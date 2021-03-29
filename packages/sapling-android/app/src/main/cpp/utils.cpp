#include "utils.h"

const unsigned char *jbyteArray_to_uchar(JNIEnv *env, jbyteArray b_arr, size_t *arr_len) {
    size_t b_arr_len = env->GetArrayLength(b_arr);
    auto *uchar_arr_buf = new unsigned char[b_arr_len];
    env->GetByteArrayRegion(b_arr, 0, b_arr_len, reinterpret_cast<jbyte*>(uchar_arr_buf));

    *arr_len = b_arr_len;
    return const_cast<unsigned char *>(uchar_arr_buf);
}

jbyteArray uchar_to_jbyteArray(JNIEnv *env, unsigned char *uchar_arr, size_t uchar_arr_len) {
    if (uchar_arr == nullptr) {
        return nullptr;
    }

    jbyteArray b_arr = env->NewByteArray(uchar_arr_len);
    auto *char_arr = const_cast<unsigned char *>(uchar_arr);

    env->SetByteArrayRegion(b_arr, 0, uchar_arr_len, reinterpret_cast<jbyte*>(char_arr));

    return b_arr;
}

void local_clean(const unsigned char *arr) {
    delete arr;
}

void ffi_clean(unsigned char *arr) {
    free(arr);
}
