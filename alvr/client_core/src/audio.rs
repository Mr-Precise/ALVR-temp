use alvr_audio::AudioDevice;
use alvr_common::{
    anyhow::{Result, bail},
    parking_lot::Mutex,
};
use alvr_session::AudioBufferingConfig;
use alvr_sockets::{StreamReceiver, StreamSender};
use ndk::audio::{
    AudioCallbackResult, AudioDirection, AudioError, AudioFormat, AudioPerformanceMode,
    AudioSharingMode, AudioStreamBuilder,
};
use std::{
    collections::VecDeque,
    slice,
    sync::{Arc, mpsc},
    time::Duration,
};

const INPUT_SAMPLES_MAX_BUFFER_COUNT: usize = 20;
const INPUT_RECV_TIMEOUT: Duration = Duration::from_millis(20);

unsafe extern "C" {
    fn alvr_create_input_stream_unprocessed(sample_rate: i32, channel_count: i32) -> *mut std::ffi::c_void;
    fn alvr_read_input_samples(
        stream: *mut std::ffi::c_void,
        buffer: *mut i16,
        frames: i32,
        timeout_us: i64,
    ) -> i32;
}

#[allow(unused_variables)]
pub fn record_audio_blocking(
    is_running: Arc<dyn Fn() -> bool + Send + Sync>,
    mut sender: StreamSender<()>,
    device: &AudioDevice,
    channels_count: u16,
    mute: bool,
) -> Result<()> {
    assert_eq!(
        channels_count, 1,
        "This code only supports mono microphone input"
    );

    let sample_rate = device.input_sample_rate()?;

    let error = Arc::new(Mutex::new(None::<AudioError>));

    let (samples_sender, samples_receiver) =
        mpsc::sync_channel::<Vec<u8>>(INPUT_SAMPLES_MAX_BUFFER_COUNT);

    let stream_ptr = unsafe { alvr_create_input_stream_unprocessed(sample_rate as i32, 1) };
    if stream_ptr.is_null() {
        bail!("Failed to create input stream with unprocessed preset");
    }

    let frames_per_buffer = 256;
    let timeout_us = 20_000;

    while is_running() {
        let mut buf = vec![0i16; frames_per_buffer];
        let read = unsafe {
            alvr_read_input_samples(
                stream_ptr,
                buf.as_mut_ptr(),
                frames_per_buffer as i32,
                timeout_us,
            )
        };

        if read > 0 {
            let sample_buffer = bytemuck::cast_slice(&buf[..read as usize]).to_vec();
            samples_sender.send(sample_buffer).ok();
        }
    }

    while is_running() && error.lock().is_none() {
        while let Ok(sample_buffer) = samples_receiver.recv_timeout(INPUT_RECV_TIMEOUT) {
            let mut buffer = sender.get_buffer(&()).unwrap();
            buffer
                .get_range_mut(0, sample_buffer.len())
                .copy_from_slice(&sample_buffer);
            sender.send(buffer).ok();
        }
    }

    if let Some(e) = *error.lock() {
        return Err(e.into());
    }

    Ok(())
}

#[allow(unused_variables)]
pub fn play_audio_loop(
    is_running: impl Fn() -> bool,
    device: &AudioDevice,
    channels_count: u16,
    sample_rate: u32,
    config: AudioBufferingConfig,
    receiver: &mut StreamReceiver<()>,
) -> Result<()> {
    assert_eq!(channels_count, 2, "This code only supports stereo output");

    if sample_rate < 8000 {
        bail!("Invalid audio sample rate");
    }

    let batch_frames_count = sample_rate as usize * config.batch_ms as usize / 1000;
    let average_buffer_frames_count =
        sample_rate as usize * config.average_buffering_ms as usize / 1000;

    let sample_buffer = Arc::new(Mutex::new(VecDeque::new()));
    let error = Arc::new(Mutex::new(None));

    let stream = AudioStreamBuilder::new()?
        .direction(AudioDirection::Output)
        .channel_count(2)
        .sample_rate(sample_rate as _)
        .format(AudioFormat::PCM_Float)
        .frames_per_data_callback(batch_frames_count as _)
        .performance_mode(AudioPerformanceMode::LowLatency)
        .sharing_mode(AudioSharingMode::Shared)
        .data_callback({
            let sample_buffer = Arc::clone(&sample_buffer);
            Box::new(move |_, data_ptr, frames_count| {
                assert!(frames_count == batch_frames_count as i32);

                let samples = alvr_audio::get_next_frame_batch(
                    &mut *sample_buffer.lock(),
                    2,
                    batch_frames_count as _,
                );

                let out_frames = unsafe {
                    slice::from_raw_parts_mut(data_ptr as *mut f32, frames_count as usize * 2)
                };
                out_frames.copy_from_slice(&samples);

                AudioCallbackResult::Continue
            })
        })
        .error_callback({
            let error = Arc::clone(&error);
            Box::new(move |_, e| *error.lock() = Some(e))
        })
        .open_stream()?;

    if stream.channel_count() != 2
        || stream.sample_rate() != sample_rate as i32
        || stream.format() != AudioFormat::PCM_Float
        || stream.frames_per_data_callback() != Some(batch_frames_count as _)
    {
        bail!("Invalid audio configuration");
    }

    stream.request_start()?;

    alvr_audio::receive_samples_loop(
        || is_running() && error.lock().is_none(),
        receiver,
        sample_buffer,
        2,
        batch_frames_count,
        average_buffer_frames_count,
    )
    .ok();

    stream.request_stop()?;

    if let Some(e) = *error.lock() {
        return Err(e.into());
    }

    Ok(())
}
