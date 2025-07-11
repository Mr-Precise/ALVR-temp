#include <aaudio/AAudio.h>
#include <stdint.h>
#include <string.h>

AAudioStream* alvr_create_input_stream_unprocessed(int sample_rate, int channel_count) {
    AAudioStreamBuilder* builder = NULL;
    AAudioStream* stream = NULL;

    if (AAudio_createStreamBuilder(&builder) != AAUDIO_OK) {
        return NULL;
    }

    AAudioStreamBuilder_setDirection(builder, AAUDIO_DIRECTION_INPUT);
    AAudioStreamBuilder_setPerformanceMode(builder, AAUDIO_PERFORMANCE_MODE_LOW_LATENCY);
    AAudioStreamBuilder_setSharingMode(builder, AAUDIO_SHARING_MODE_SHARED);
    AAudioStreamBuilder_setFormat(builder, AAUDIO_FORMAT_PCM_I16);

    AAudioStreamBuilder_setSampleRate(builder, sample_rate);
    AAudioStreamBuilder_setChannelCount(builder, channel_count);

    if (AAudioStreamBuilder_openStream(builder, &stream) != AAUDIO_OK) {
        AAudioStreamBuilder_delete(builder);
        return NULL;
    }

    AAudioStreamBuilder_delete(builder);

    if (AAudioStream_requestStart(stream) != AAUDIO_OK) {
        AAudioStream_close(stream);
        return NULL;
    }

    return stream;
}

int alvr_read_input_samples(AAudioStream* stream, int16_t* buffer, int frames, int64_t timeout_us) {
    return AAudioStream_read(stream, buffer, frames, timeout_us);
}
