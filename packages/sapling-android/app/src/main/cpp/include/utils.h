#ifndef SAPLING_UTILS_H
#define SAPLING_UTILS_H

#include <jni.h>
#include <cstdlib>

const unsigned char *jbyteArray_to_uchar(JNIEnv *env, jbyteArray b_arr, size_t *arr_len);
jbyteArray uchar_to_jbyteArray(JNIEnv *env, unsigned char *uchar_arr, size_t uchar_arr_len);

void local_clean(const unsigned char *arr);
void ffi_clean(unsigned char *arr);

#endif //SAPLING_UTILS_H
