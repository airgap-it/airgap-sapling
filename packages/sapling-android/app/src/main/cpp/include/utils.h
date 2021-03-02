#ifndef SAPLING_UTILS_H
#define SAPLING_UTILS_H

#include <jni.h>

size_t jbyteArray_to_uchar(JNIEnv *env, jbyteArray b_arr, const unsigned char *&uchar_arr);
jbyteArray uchar_to_jbyteArray(JNIEnv *env, const unsigned char *uchar_arr, size_t uchar_arr_len);

#endif //SAPLING_UTILS_H
