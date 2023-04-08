#ifdef __ANDROID__
#include <android/log.h>
#include <stdarg.h>

extern void lwip_android_log(const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);
    __android_log_vprint(ANDROID_LOG_DEBUG, "lwip-rs", fmt, args);
    va_end(args);
}
#endif
