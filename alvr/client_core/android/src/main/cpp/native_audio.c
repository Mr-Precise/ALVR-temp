#include <aaudio/AAudio.h>
#include <stdint.h>
#include <string.h>

AAudioStream* alvr_create_input_stream_unprocessed(int sample_rate, int channel_count) {
    AAudioStreamBuilder* builder;
    AAudioStream* stream;

    aaudio_create_stream_builder(&builder);
    AAudioStreamBuilder_setDirection(builder, AAUDIO_DIRECTION_INPUT);
    AAudioStreamBuilder_setPerformanceMode(builder, AAUDIO_PERFORMANCE_MODE_LOW_LATENCY);
    AAudioStreamBuilder_setSharingMode(builder, AAUDIO_SHARING_MODE_SHARED);
    AAudioStreamBuilder_setFormat(builder, AAUDIO_FORMAT_PCM_I16);
    AAudioStreamBuilder_setInputPreset(builder, AAUDIO_INPUT_PRESET_UNPROCESSED);
    AAudioStreamBuilder_setSampleRate(builder, sample_rate);
    AAudioStreamBuilder_setChannelCount(builder, channel_count);

    if (AAudioStreamBuilder_openStream(builder, &stream) != AAUDIO_OK) {
        return NULL;
    }

    aaudio_stream_request_start(stream);
    aaudio_stream_close(builder); // clean up builder

    return stream;
}

// Чтение аудиоданных в буфер (blocking read)
int alvr_read_input_samples(AAudioStream* stream, int16_t* buffer, int frames, int64_t timeout_us) {
    return AAudioStream_read(stream, buffer, frames, timeout_us);
}
