#include "utils.h"

size_t jbyteArray_to_uchar(JNIEnv *env, jbyteArray b_arr, const unsigned char *&uchar_arr) {
    size_t b_arr_len = env->GetArrayLength(b_arr);
    auto *uchar_arr_buf = new unsigned char[b_arr_len];
    env->GetByteArrayRegion(b_arr, 0, b_arr_len, reinterpret_cast<jbyte*>(uchar_arr_buf));

    uchar_arr = const_cast<unsigned char *>(uchar_arr_buf);
    return b_arr_len;
}

jbyteArray uchar_to_jbyteArray(JNIEnv *env, const unsigned char *uchar_arr, size_t uchar_arr_len) {
    jbyteArray b_arr = env->NewByteArray(uchar_arr_len);
    auto *char_arr = const_cast<unsigned char *>(uchar_arr);

    env->SetByteArrayRegion(b_arr, 0, uchar_arr_len, reinterpret_cast<jbyte*>(char_arr));

    return b_arr;
}
