#include "sapling.h"

/******** Commitment ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extComputeCmu(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jaddr,
        jlong jval,
        jbyteArray jrcm) {
    const unsigned char *addr;
    size_t addr_len = jbyteArray_to_uchar(env, jaddr, addr);

    auto val = (uint64_t) jval;

    const unsigned char *rcm;
    size_t rcm_len = jbyteArray_to_uchar(env, jrcm, rcm);

    const unsigned char *cmu;
    size_t cmu_len = c_compute_cmu(addr, addr_len, val, rcm, rcm_len, cmu);

    return cmu_len != 0 ? uchar_to_jbyteArray(env, cmu, cmu_len) : nullptr;
}

/******** Init ********/

extern "C"
JNIEXPORT jboolean JNICALL
Java_it_airgap_sapling_Sapling_extInitParameters(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jspendParams,
        jbyteArray joutputParams) {
    const unsigned char *s_params;
    size_t s_params_len = jbyteArray_to_uchar(env, jspendParams, s_params);

    const unsigned char *o_params;
    size_t o_params_len = jbyteArray_to_uchar(env, joutputParams, o_params);

    return c_init_params(s_params, s_params_len, o_params, o_params_len);
}

/******** Key Agreement ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extKeyAgreement(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jp,
        jbyteArray jsk) {
    const unsigned char *p;
    size_t p_len = jbyteArray_to_uchar(env, jp, p);

    const unsigned char *sk;
    size_t sk_len = jbyteArray_to_uchar(env, jsk, sk);

    const unsigned char *ka;
    size_t ka_len = c_key_agreement(p, p_len, sk, sk_len, ka);

    return ka_len != 0 ? uchar_to_jbyteArray(env, ka, ka_len) : nullptr;
}

/******** Merkle Tree ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extMerkleHash(
        JNIEnv *env,
        jobject /* this */,
        jlong jdepth,
        jbyteArray jlhs,
        jbyteArray jrhs) {
    auto depth = (uint64_t) jdepth;

    const unsigned char *lhs;
    size_t lhs_len = jbyteArray_to_uchar(env, jlhs, lhs);

    const unsigned char *rhs;
    size_t rhs_len = jbyteArray_to_uchar(env, jrhs, rhs);

    const unsigned char *m_hash;
    size_t m_hash_len = c_merkle_hash(depth, lhs, lhs_len, rhs, rhs_len, m_hash);

    return m_hash_len != 0 ? uchar_to_jbyteArray(env, m_hash, m_hash_len) : nullptr;
}

/******** Nullifier ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extComputeNullifierWithXfvk(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jxfvk,
        jbyteArray jaddr,
        jlong jval,
        jbyteArray jrcm,
        jlong jpos) {
    const unsigned char *xfvk;
    size_t xfvk_len = jbyteArray_to_uchar(env, jxfvk, xfvk);

    const unsigned char *addr;
    size_t addr_len = jbyteArray_to_uchar(env, jaddr, addr);

    auto val = (uint64_t) jval;

    const unsigned char *rcm;
    size_t rcm_len = jbyteArray_to_uchar(env, jrcm, rcm);

    auto pos = (uint64_t) jpos;

    const unsigned char *nullfier;
    size_t nullifier_len = c_compute_nullifier_with_xfvk(xfvk, xfvk_len, addr, addr_len, val, rcm, rcm_len, pos, nullfier);

    return nullifier_len != 0 ? uchar_to_jbyteArray(env, nullfier, nullifier_len) : nullptr;
}

/******** Output Description ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extOutputDescriptionFromXfvk(
        JNIEnv *env, 
        jobject /* this */,
        jlong jctx,
        jbyteArray jxfvk,
        jbyteArray jaddr,
        jbyteArray jrcm,
        jlong jval) {
    void *ctx = (void *) jctx;
    
    const unsigned char *xfvk;
    size_t xfvk_len = jbyteArray_to_uchar(env, jxfvk, xfvk);

    const unsigned char *addr;
    size_t addr_len = jbyteArray_to_uchar(env, jaddr, addr);

    const unsigned char *rcm;
    size_t rcm_len = jbyteArray_to_uchar(env, jrcm, rcm);

    auto val = (uint64_t) jval;
    
    const unsigned char *o_desc;
    size_t o_desc_len = c_output_description_from_xfvk(ctx, xfvk, xfvk_len, addr, addr_len, rcm, rcm_len, val, o_desc);
    
    return o_desc_len != 0 ? uchar_to_jbyteArray(env, o_desc, o_desc_len) : nullptr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extOutputDescriptionFromXfvkWithMemo(
        JNIEnv *env,
        jobject /* this */,
        jlong jctx,
        jbyteArray jxfvk,
        jbyteArray jaddr,
        jbyteArray jrcm,
        jlong jval,
        jbyteArray jmemo) {
    void *ctx = (void *) jctx;

    const unsigned char *xfvk;
    size_t xfvk_len = jbyteArray_to_uchar(env, jxfvk, xfvk);

    const unsigned char *addr;
    size_t addr_len = jbyteArray_to_uchar(env, jaddr, addr);

    const unsigned char *rcm;
    size_t rcm_len = jbyteArray_to_uchar(env, jrcm, rcm);

    auto val = (uint64_t) jval;

    const unsigned char *memo;
    size_t memo_len = jbyteArray_to_uchar(env, jmemo, memo);

    const unsigned char *o_desc;
    size_t o_desc_len = c_output_description_from_xfvk_with_memo(
            ctx,
            xfvk,
            xfvk_len,
            addr,
            addr_len,
            rcm,
            rcm_len,
            val,
            memo,
            memo_len,
            o_desc
    );

    return o_desc_len != 0 ? uchar_to_jbyteArray(env, o_desc, o_desc_len) : nullptr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extOutputDescriptionFromOvk(
        JNIEnv *env, 
        jobject /* this */,
        jlong jctx,
        jbyteArray jovk,
        jbyteArray jaddr,
        jbyteArray jrcm,
        jlong jval) {
    void *ctx = (void *) jctx;

    const unsigned char *ovk;
    size_t ovk_len = jbyteArray_to_uchar(env, jovk, ovk);

    const unsigned char *addr;
    size_t addr_len = jbyteArray_to_uchar(env, jaddr, addr);

    const unsigned char *rcm;
    size_t rcm_len = jbyteArray_to_uchar(env, jrcm, rcm);

    auto val = (uint64_t) jval;

    const unsigned char *o_desc;
    size_t o_desc_len = c_output_description_from_ovk(ctx, ovk, ovk_len, addr, addr_len, rcm, rcm_len, val, o_desc);

    return o_desc_len != 0 ? uchar_to_jbyteArray(env, o_desc, o_desc_len) : nullptr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extPartialOutputDescription(
        JNIEnv *env,
        jobject /* this */,
        jlong jctx,
        jbyteArray jaddr,
        jbyteArray jrcm,
        jbyteArray jesk,
        jlong jval) {
    void *ctx = (void *) jctx;

    const unsigned char *addr;
    size_t addr_len = jbyteArray_to_uchar(env, jaddr, addr);

    const unsigned char *rcm;
    size_t rcm_len = jbyteArray_to_uchar(env, jrcm, rcm);

    const unsigned char *esk;
    size_t esk_len = jbyteArray_to_uchar(env, jesk, esk);

    auto val = (uint64_t) jval;

    const unsigned char *o_desc;
    size_t o_desc_len = c_partial_output_description(ctx, addr, addr_len, rcm, rcm_len, esk, esk_len, val, o_desc);

    return o_desc_len != 0 ? uchar_to_jbyteArray(env, o_desc, o_desc_len) : nullptr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extDeriveEpkFromEsk(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jdiv,
        jbyteArray jesk) {
    const unsigned char *div;
    size_t div_len = jbyteArray_to_uchar(env, jdiv, div);

    const unsigned char *esk;
    size_t esk_len = jbyteArray_to_uchar(env, jesk, esk);

    const unsigned char *epk;
    size_t epk_len = c_derive_epk_from_esk(div, div_len, esk, esk_len, epk);
    
    return epk_len != 0 ? uchar_to_jbyteArray(env, epk, epk_len) : nullptr;
}

/******** Payment Address ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extDefaultPaymentAddressFromXfvk(
        JNIEnv *env, 
        jobject /* this */,
        jbyteArray jxfvk) {
    const unsigned char *xfvk;
    size_t xfvk_len = jbyteArray_to_uchar(env, jxfvk, xfvk);
    
    const unsigned char *addr;
    size_t addr_len = c_default_payment_address_from_xfvk(xfvk, xfvk_len, addr);
    
    return addr_len != 0 ? uchar_to_jbyteArray(env, addr, addr_len) : nullptr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extNextPaymentAddressFromXfvk(
        JNIEnv *env, 
        jobject /* this */,
        jbyteArray jxfvk,
        jbyteArray jidx) {
    const unsigned char *xfvk;
    size_t xfvk_len = jbyteArray_to_uchar(env, jxfvk, xfvk);

    const unsigned char *idx;
    size_t idx_len = jbyteArray_to_uchar(env, jidx, idx);

    const unsigned char *addr;
    size_t addr_len = c_next_payment_address_from_xfvk(xfvk, xfvk_len, idx, idx_len, addr);

    return addr_len != 0 ? uchar_to_jbyteArray(env, addr, addr_len) : nullptr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extPaymentAddressFromXfvk(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jxfvk,
        jbyteArray jidx) {
    const unsigned char *xfvk;
    size_t xfvk_len = jbyteArray_to_uchar(env, jxfvk, xfvk);

    const unsigned char *idx;
    size_t idx_len = jbyteArray_to_uchar(env, jidx, idx);

    const unsigned char *addr;
    size_t addr_len = c_payment_address_from_xfvk(xfvk, xfvk_len, idx, idx_len, addr);

    return addr_len != 0 ? uchar_to_jbyteArray(env, addr, addr_len) : nullptr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extPaymentAddressFromIvk(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jivk,
        jbyteArray jdiv) {
    const unsigned char *ivk;
    size_t ivk_len = jbyteArray_to_uchar(env, jivk, ivk);

    const unsigned char *div;
    size_t div_len = jbyteArray_to_uchar(env, jdiv, div);

    const unsigned char *addr;
    size_t addr_len = c_payment_address_from_xfvk(ivk, ivk_len, div, div_len, addr);

    return addr_len != 0 ? uchar_to_jbyteArray(env, addr, addr_len) : nullptr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extDiversifierFromPaymentAddress(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jaddr) {
    const unsigned char *addr;
    size_t addr_len = jbyteArray_to_uchar(env, jaddr, addr);

    const unsigned char *div;
    size_t div_len = c_diversifier_from_payment_address(addr, addr_len, div);

    return div_len != 0 ? uchar_to_jbyteArray(env, div, div_len) : nullptr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extPkdFromPaymentAddress(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jaddr) {
    const unsigned char *addr;
    size_t addr_len = jbyteArray_to_uchar(env, jaddr, addr);

    const unsigned char *pkd;
    size_t pkd_len = c_diversifier_from_payment_address(addr, addr_len, pkd);

    return pkd_len != 0 ? uchar_to_jbyteArray(env, pkd, pkd_len) : nullptr;
}

/******** Proving Context ********/

extern "C"
JNIEXPORT jlong JNICALL
Java_it_airgap_sapling_Sapling_extInitProvingContext(
        JNIEnv *env,
        jobject /* this */) {
    return (jlong) c_init_proving_context();
}

extern "C"
JNIEXPORT void JNICALL
Java_it_airgap_sapling_Sapling_extDropProvingContext(
        JNIEnv *env,
        jobject /* this */,
        jlong jctx) {
    c_drop_proving_context((void *) jctx);
}

/******** Rand ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extRandR(JNIEnv *env, jobject /* this */) {
    const unsigned char *r;
    size_t r_len = c_rand_r(r);

    return r_len != 0 ? uchar_to_jbyteArray(env, r, r_len) : nullptr;
}

/******** Signature ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extCreateBindingSignature(
        JNIEnv *env,
        jobject /* this */,
        jlong jctx,
        jlong jbal,
        jbyteArray jsighash) {
    void *ctx = (void *) jctx;
    auto bal = (int64_t) jbal;

    const unsigned char *sighash;
    size_t sighash_len = jbyteArray_to_uchar(env, jsighash, sighash);

    const unsigned char *sig;
    size_t sig_len = c_binding_signature(ctx, bal, sighash, sighash_len, sig);

    return sig_len != 0 ? uchar_to_jbyteArray(env, sig, sig_len) : nullptr;
}

/******** Spend Description ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extSpendDescriptionFromXsk(
        JNIEnv *env,
        jobject /* this */,
        jlong jctx,
        jbyteArray jxsk,
        jbyteArray jaddr,
        jbyteArray jrcm,
        jbyteArray jar,
        jlong jval,
        jbyteArray janchor,
        jbyteArray jmerklePath) {
    void *ctx = (void *) jctx;

    const unsigned char *xsk;
    size_t xsk_len = jbyteArray_to_uchar(env, jxsk, xsk);

    const unsigned char *addr;
    size_t addr_len = jbyteArray_to_uchar(env, jaddr, addr);

    const unsigned char *rcm;
    size_t rcm_len = jbyteArray_to_uchar(env, jrcm, rcm);

    const unsigned char *ar;
    size_t ar_len = jbyteArray_to_uchar(env, jar, ar);

    auto val = (uint64_t) jval;

    const unsigned char *anchor;
    size_t anchor_len = jbyteArray_to_uchar(env, janchor, anchor);

    const unsigned char *merkle_path;
    size_t merkle_path_len = jbyteArray_to_uchar(env, jmerklePath, merkle_path);

    const unsigned char *s_desc;
    size_t s_desc_len = c_spend_description_from_xsk(
            ctx,
            xsk,
            xsk_len,
            addr,
            addr_len,
            rcm,
            rcm_len,
            ar,
            ar_len,
            val,
            anchor,
            anchor_len,
            merkle_path,
            merkle_path_len,
            s_desc
    );

    return s_desc_len != 0 ? uchar_to_jbyteArray(env, s_desc, s_desc_len) : nullptr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extSignSpendDescriptionWithXsk(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray js_desc,
        jbyteArray jxsk,
        jbyteArray jar,
        jbyteArray jsighash) {
    const unsigned char *s_desc;
    size_t s_desc_len = jbyteArray_to_uchar(env, js_desc, s_desc);

    const unsigned char *xsk;
    size_t xsk_len = jbyteArray_to_uchar(env, jxsk, xsk);

    const unsigned char *ar;
    size_t ar_len = jbyteArray_to_uchar(env, jar, ar);

    const unsigned char *sighash;
    size_t sighash_len = jbyteArray_to_uchar(env, jsighash, sighash);

    const unsigned char *signed_s_desc;
    size_t signed_s_desc_len = c_sign_spend_description_with_xsk(
            s_desc,
            s_desc_len,
            xsk,
            xsk_len,
            ar,
            ar_len,
            sighash,
            sighash_len,
            signed_s_desc
    );

    return sighash_len != 0 ? uchar_to_jbyteArray(env, signed_s_desc, signed_s_desc_len) : nullptr;
}

/******** Spending Key ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extXsk(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jseed,
        jstring jd_path) {
    const unsigned char *seed;
    size_t seed_len = jbyteArray_to_uchar(env, jseed, seed);

    const char *d_path = env->GetStringUTFChars(jd_path, nullptr);

    const unsigned char *xsk;
    size_t xsk_len = c_xsk(seed, seed_len, d_path, xsk);

    return xsk_len != 0 ? uchar_to_jbyteArray(env, xsk, xsk_len) : nullptr;
}

/******** Viewing Key ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extXfvk(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jseed,
        jstring jd_path) {
    const unsigned char *seed;
    size_t seed_len = jbyteArray_to_uchar(env, jseed, seed);

    const char *d_path = env->GetStringUTFChars(jd_path, nullptr);

    const unsigned char *xfvk;
    size_t xfvk_len = c_xfvk(seed, seed_len, d_path, xfvk);

    return xfvk_len != 0 ? uchar_to_jbyteArray(env, xfvk, xfvk_len) : nullptr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extXfvkFromXsk(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jxsk) {
    const unsigned char *xsk;
    size_t xsk_len = jbyteArray_to_uchar(env, jxsk, xsk);

    const unsigned char *xfvk;
    size_t xfvk_len = c_xfvk_from_xsk(xsk, xsk_len, xfvk);

    return xfvk_len != 0 ? uchar_to_jbyteArray(env, xfvk, xfvk_len) : nullptr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extOvkFromXfvk(JNIEnv *env, jobject /* this */, jbyteArray jxfvk) {
    const unsigned char *xfvk;
    size_t xfvk_len = jbyteArray_to_uchar(env, jxfvk, xfvk);

    const unsigned char *ovk;
    size_t ovk_len = c_xfvk_from_xsk(xfvk, xfvk_len, ovk);

    return ovk_len != 0 ? uchar_to_jbyteArray(env, ovk, ovk_len) : nullptr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extXfvkToIvk(JNIEnv *env, jobject /* this */, jbyteArray jxfvk) {
    const unsigned char *xfvk;
    size_t xfvk_len = jbyteArray_to_uchar(env, jxfvk, xfvk);

    const unsigned char *ivk;
    size_t ivk_len = c_xfvk_to_ivk(xfvk, xfvk_len, ivk);

    return ivk_len != 0 ? uchar_to_jbyteArray(env, ivk, ivk_len) : nullptr;
}