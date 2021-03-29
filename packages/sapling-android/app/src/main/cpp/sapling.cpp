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
    size_t addr_len;
    const unsigned char *addr = jbyteArray_to_uchar(env, jaddr, &addr_len);

    auto val = (uint64_t) jval;

    size_t rcm_len;
    const unsigned char *rcm = jbyteArray_to_uchar(env, jrcm, &rcm_len);

    size_t cmu_len;
    unsigned char *cmu = c_compute_cmu(addr, addr_len, val, rcm, rcm_len, &cmu_len);
    jbyteArray jcmu = uchar_to_jbyteArray(env, cmu, cmu_len);

    local_clean(addr);
    local_clean(rcm);
    ffi_clean(cmu);

    return jcmu;
}

/******** Init ********/

extern "C"
JNIEXPORT jboolean JNICALL
Java_it_airgap_sapling_Sapling_extInitParameters(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jspendParams,
        jbyteArray joutputParams) {
    size_t s_params_len;
    const unsigned char *s_params = jbyteArray_to_uchar(env, jspendParams, &s_params_len);

    size_t o_params_len;
    const unsigned char *o_params = jbyteArray_to_uchar(env, joutputParams, &o_params_len);

    jboolean jinit = c_init_params(s_params, s_params_len, o_params, o_params_len);

    local_clean(s_params);
    local_clean(o_params);

    return jinit;
}

/******** Key Agreement ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extKeyAgreement(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jp,
        jbyteArray jsk) {
    size_t p_len;
    const unsigned char *p = jbyteArray_to_uchar(env, jp, &p_len);

    size_t sk_len;
    const unsigned char *sk = jbyteArray_to_uchar(env, jsk, &sk_len);

    size_t ka_len;
    unsigned char *ka = c_key_agreement(p, p_len, sk, sk_len, &ka_len);
    jbyteArray jka = uchar_to_jbyteArray(env, ka, ka_len);

    local_clean(p);
    local_clean(sk);
    ffi_clean(ka);

    return jka;
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
    auto depth = (size_t) jdepth;

    size_t lhs_len;
    const unsigned char *lhs = jbyteArray_to_uchar(env, jlhs, &lhs_len);

    size_t rhs_len;
    const unsigned char *rhs = jbyteArray_to_uchar(env, jrhs, &rhs_len);

    size_t m_hash_len;
    unsigned char *m_hash = c_merkle_hash(depth, lhs, lhs_len, rhs, rhs_len, &m_hash_len);
    jbyteArray jm_hash = uchar_to_jbyteArray(env, m_hash, m_hash_len);

    local_clean(lhs);
    local_clean(rhs);
    ffi_clean(m_hash);

    return jm_hash;
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
    size_t xfvk_len;
    const unsigned char *xfvk = jbyteArray_to_uchar(env, jxfvk, &xfvk_len);

    size_t addr_len;
    const unsigned char *addr = jbyteArray_to_uchar(env, jaddr, &addr_len);

    auto val = (uint64_t) jval;

    size_t rcm_len;
    const unsigned char *rcm = jbyteArray_to_uchar(env, jrcm, &rcm_len);

    auto pos = (uint64_t) jpos;

    size_t nullifier_len;
    unsigned char *nullifier = c_compute_nullifier_with_xfvk(xfvk, xfvk_len, addr, addr_len, val, rcm, rcm_len, pos, &nullifier_len);
    jbyteArray jnullifier = uchar_to_jbyteArray(env, nullifier, nullifier_len);

    local_clean(xfvk);
    local_clean(addr);
    local_clean(rcm);
    ffi_clean(nullifier);
    
    return jnullifier;
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
    
    size_t xfvk_len;
    const unsigned char *xfvk = jbyteArray_to_uchar(env, jxfvk, &xfvk_len);

    size_t addr_len;
    const unsigned char *addr = jbyteArray_to_uchar(env, jaddr, &addr_len);

    size_t rcm_len;
    const unsigned char *rcm = jbyteArray_to_uchar(env, jrcm, &rcm_len);

    auto val = (uint64_t) jval;
    
    size_t o_desc_len;
    unsigned char *o_desc = c_output_description_from_xfvk(ctx, xfvk, xfvk_len, addr, addr_len, rcm, rcm_len, val, &o_desc_len);
    jbyteArray jo_desc = uchar_to_jbyteArray(env, o_desc, o_desc_len);

    local_clean(xfvk);
    local_clean(addr);
    local_clean(rcm);
    ffi_clean(o_desc);
    
    return jo_desc;
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

    size_t xfvk_len;
    const unsigned char *xfvk = jbyteArray_to_uchar(env, jxfvk, &xfvk_len);

    size_t addr_len;
    const unsigned char *addr = jbyteArray_to_uchar(env, jaddr, &addr_len);

    size_t rcm_len;
    const unsigned char *rcm = jbyteArray_to_uchar(env, jrcm, &rcm_len);

    auto val = (uint64_t) jval;

    size_t memo_len;
    const unsigned char *memo = jbyteArray_to_uchar(env, jmemo, &memo_len);

    size_t o_desc_len;
    unsigned char *o_desc = c_output_description_from_xfvk_with_memo(
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
            &o_desc_len
    );
    jbyteArray jo_desc = uchar_to_jbyteArray(env, o_desc, o_desc_len);
    
    local_clean(xfvk);
    local_clean(addr);
    local_clean(rcm);
    local_clean(memo);
    ffi_clean(o_desc);
    
    return jo_desc;
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

    size_t ovk_len;
    const unsigned char *ovk = jbyteArray_to_uchar(env, jovk, &ovk_len);

    size_t addr_len;
    const unsigned char *addr = jbyteArray_to_uchar(env, jaddr, &addr_len);

    size_t rcm_len;
    const unsigned char *rcm = jbyteArray_to_uchar(env, jrcm, &rcm_len);

    auto val = (uint64_t) jval;

    size_t o_desc_len;
    unsigned char *o_desc = c_output_description_from_ovk(ctx, ovk, ovk_len, addr, addr_len, rcm, rcm_len, val, &o_desc_len);
    jbyteArray jo_desc = uchar_to_jbyteArray(env, o_desc, o_desc_len);

    local_clean(ovk);
    local_clean(addr);
    local_clean(rcm);
    ffi_clean(o_desc);

    return jo_desc;
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

    size_t addr_len;
    const unsigned char *addr = jbyteArray_to_uchar(env, jaddr, &addr_len);

    size_t rcm_len;
    const unsigned char *rcm = jbyteArray_to_uchar(env, jrcm, &rcm_len);

    size_t esk_len;
    const unsigned char *esk = jbyteArray_to_uchar(env, jesk, &esk_len);

    auto val = (uint64_t) jval;

    size_t o_desc_len;
    unsigned char *o_desc = c_partial_output_description(ctx, addr, addr_len, rcm, rcm_len, esk, esk_len, val, &o_desc_len);
    jbyteArray jo_desc = uchar_to_jbyteArray(env, o_desc, o_desc_len);

    local_clean(addr);
    local_clean(rcm);
    local_clean(esk);
    ffi_clean(o_desc);

    return jo_desc;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extDeriveEpkFromEsk(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jdiv,
        jbyteArray jesk) {
    size_t div_len;
    const unsigned char *div = jbyteArray_to_uchar(env, jdiv, &div_len);

    size_t esk_len;
    const unsigned char *esk = jbyteArray_to_uchar(env, jesk, &esk_len);

    size_t epk_len;
    unsigned char *epk = c_derive_epk_from_esk(div, div_len, esk, esk_len, &epk_len);
    jbyteArray jepk = uchar_to_jbyteArray(env, epk, epk_len);
    
    local_clean(div);
    local_clean(esk);
    ffi_clean(epk);
    
    return jepk;
}

/******** Payment Address ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extDefaultPaymentAddressFromXfvk(
        JNIEnv *env, 
        jobject /* this */,
        jbyteArray jxfvk) {
    size_t xfvk_len;
    const unsigned char *xfvk = jbyteArray_to_uchar(env, jxfvk, &xfvk_len);
    
    size_t addr_len;
    unsigned char *addr = c_default_payment_address_from_xfvk(xfvk, xfvk_len, &addr_len);
    jbyteArray jaddr = uchar_to_jbyteArray(env, addr, addr_len);
    
    local_clean(xfvk);
    ffi_clean(addr);
    
    return jaddr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extNextPaymentAddressFromXfvk(
        JNIEnv *env, 
        jobject /* this */,
        jbyteArray jxfvk,
        jbyteArray jidx) {
    size_t xfvk_len;
    const unsigned char *xfvk = jbyteArray_to_uchar(env, jxfvk, &xfvk_len);

    size_t idx_len;
    const unsigned char *idx = jbyteArray_to_uchar(env, jidx, &idx_len);

    size_t addr_len;
    unsigned char *addr = c_next_payment_address_from_xfvk(xfvk, xfvk_len, idx, idx_len, &addr_len);
    jbyteArray jaddr = uchar_to_jbyteArray(env, addr, addr_len);
    
    local_clean(xfvk);
    ffi_clean(addr);
    
    return jaddr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extPaymentAddressFromXfvk(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jxfvk,
        jbyteArray jidx) {
    size_t xfvk_len;
    const unsigned char *xfvk = jbyteArray_to_uchar(env, jxfvk, &xfvk_len);

    size_t idx_len;
    const unsigned char *idx = jbyteArray_to_uchar(env, jidx, &idx_len);

    size_t addr_len;
    unsigned char *addr = c_payment_address_from_xfvk(xfvk, xfvk_len, idx, idx_len, &addr_len);
    jbyteArray jaddr = uchar_to_jbyteArray(env, addr, addr_len);
    
    local_clean(xfvk);
    local_clean(idx);
    ffi_clean(addr);
    
    return jaddr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extPaymentAddressFromIvk(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jivk,
        jbyteArray jdiv) {
    size_t ivk_len;
    const unsigned char *ivk = jbyteArray_to_uchar(env, jivk, &ivk_len);

    size_t div_len;
    const unsigned char *div = jbyteArray_to_uchar(env, jdiv, &div_len);

    size_t addr_len;
    unsigned char *addr = c_payment_address_from_xfvk(ivk, ivk_len, div, div_len, &addr_len);
    jbyteArray jaddr = uchar_to_jbyteArray(env, addr, addr_len);

    local_clean(ivk);
    local_clean(div);
    ffi_clean(addr);

    return jaddr;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extDiversifierFromPaymentAddress(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jaddr) {
    size_t addr_len;
    const unsigned char *addr = jbyteArray_to_uchar(env, jaddr, &addr_len);

    size_t div_len;
    unsigned char *div = c_diversifier_from_payment_address(addr, addr_len, &div_len);
    jbyteArray jdiv = uchar_to_jbyteArray(env, div, div_len);
    
    local_clean(addr);
    ffi_clean(div);
    
    return jdiv;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extPkdFromPaymentAddress(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jaddr) {
    size_t addr_len;
    const unsigned char *addr = jbyteArray_to_uchar(env, jaddr, &addr_len);

    size_t pkd_len;
    unsigned char *pkd = c_diversifier_from_payment_address(addr, addr_len, &pkd_len);
    jbyteArray jpkd = uchar_to_jbyteArray(env, pkd, pkd_len);

    local_clean(addr);
    ffi_clean(pkd);

    return jpkd;
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
    size_t r_len;
    unsigned char *r = c_rand_r(&r_len);
    jbyteArray jr = uchar_to_jbyteArray(env, r, r_len);

    ffi_clean(r);

    return jr;
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

    size_t sighash_len;
    const unsigned char *sighash = jbyteArray_to_uchar(env, jsighash, &sighash_len);

    size_t sig_len;
    unsigned char *sig = c_binding_signature(ctx, bal, sighash, sighash_len, &sig_len);
    jbyteArray jsig = uchar_to_jbyteArray(env, sig, sig_len);

    local_clean(sighash);
    ffi_clean(sig);

    return jsig;
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

    size_t xsk_len;
    const unsigned char *xsk = jbyteArray_to_uchar(env, jxsk, &xsk_len);

    size_t addr_len;
    const unsigned char *addr = jbyteArray_to_uchar(env, jaddr, &addr_len);

    size_t rcm_len;
    const unsigned char *rcm = jbyteArray_to_uchar(env, jrcm, &rcm_len);

    size_t ar_len;
    const unsigned char *ar = jbyteArray_to_uchar(env, jar, &ar_len);

    auto val = (uint64_t) jval;

    size_t anchor_len;
    const unsigned char *anchor = jbyteArray_to_uchar(env, janchor, &anchor_len);

    size_t merkle_path_len;
    const unsigned char *merkle_path = jbyteArray_to_uchar(env, jmerklePath, &merkle_path_len);

    size_t s_desc_len;
    unsigned char *s_desc = c_spend_description_from_xsk(
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
            &s_desc_len
    );
    jbyteArray js_desc = uchar_to_jbyteArray(env, s_desc, s_desc_len);

    local_clean(xsk);
    local_clean(addr);
    local_clean(rcm);
    local_clean(ar);
    local_clean(anchor);
    local_clean(merkle_path);
    ffi_clean(s_desc);

    return js_desc;
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
    size_t s_desc_len;
    const unsigned char *s_desc = jbyteArray_to_uchar(env, js_desc, &s_desc_len);

    size_t xsk_len;
    const unsigned char *xsk = jbyteArray_to_uchar(env, jxsk, &xsk_len);

    size_t ar_len;
    const unsigned char *ar = jbyteArray_to_uchar(env, jar, &ar_len);

    size_t sighash_len;
    const unsigned char *sighash = jbyteArray_to_uchar(env, jsighash, &sighash_len);

    size_t signed_s_desc_len;
    unsigned char *signed_s_desc = c_sign_spend_description_with_xsk(
            s_desc,
            s_desc_len,
            xsk,
            xsk_len,
            ar,
            ar_len,
            sighash,
            sighash_len,
            &signed_s_desc_len
    );
    jbyteArray jsigned_s_desc = uchar_to_jbyteArray(env, signed_s_desc, signed_s_desc_len);

    local_clean(s_desc);
    local_clean(xsk);
    local_clean(ar);
    local_clean(sighash);
    ffi_clean(signed_s_desc);

    return jsigned_s_desc;
}

/******** Spending Key ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extXsk(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jseed,
        jstring jd_path) {
    size_t seed_len;
    const unsigned char *seed = jbyteArray_to_uchar(env, jseed, &seed_len);

    const char *d_path = env->GetStringUTFChars(jd_path, nullptr);

    size_t xsk_len;
    unsigned char *xsk = c_xsk(seed, seed_len, d_path, &xsk_len);
    jbyteArray jxsk = uchar_to_jbyteArray(env, xsk, xsk_len);

    local_clean(seed);
    ffi_clean(xsk);

    return jxsk;
}

/******** Viewing Key ********/

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extXfvk(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jseed,
        jstring jd_path) {
    size_t seed_len;
    const unsigned char *seed = jbyteArray_to_uchar(env, jseed, &seed_len);

    const char *d_path = env->GetStringUTFChars(jd_path, nullptr);

    size_t xfvk_len;
    unsigned char *xfvk = c_xfvk(seed, seed_len, d_path, &xfvk_len);
    jbyteArray jxfvk = uchar_to_jbyteArray(env, xfvk, xfvk_len);

    local_clean(seed);
    ffi_clean(xfvk);

    return jxfvk;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extXfvkFromXsk(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray jxsk) {
    size_t xsk_len;
    const unsigned char *xsk = jbyteArray_to_uchar(env, jxsk, &xsk_len);

    size_t xfvk_len;
    unsigned char *xfvk = c_xfvk_from_xsk(xsk, xsk_len, &xfvk_len);
    jbyteArray jxfvk = uchar_to_jbyteArray(env, xfvk, xfvk_len);

    local_clean(xsk);
    ffi_clean(xfvk);

    return jxfvk;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extOvkFromXfvk(JNIEnv *env, jobject /* this */, jbyteArray jxfvk) {
    size_t xfvk_len;
    const unsigned char *xfvk = jbyteArray_to_uchar(env, jxfvk, &xfvk_len);

    size_t ovk_len;
    unsigned char *ovk = c_xfvk_from_xsk(xfvk, xfvk_len, &ovk_len);
    jbyteArray jovk = uchar_to_jbyteArray(env, ovk, ovk_len);

    local_clean(xfvk);
    ffi_clean(ovk);

    return jovk;
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_it_airgap_sapling_Sapling_extXfvkToIvk(JNIEnv *env, jobject /* this */, jbyteArray jxfvk) {
    size_t xfvk_len;
    const unsigned char *xfvk = jbyteArray_to_uchar(env, jxfvk, &xfvk_len);

    size_t ivk_len;
    unsigned char *ivk = c_xfvk_to_ivk(xfvk, xfvk_len, &ivk_len);
    jbyteArray jivk = uchar_to_jbyteArray(env, ivk, ivk_len);

    local_clean(xfvk);
    ffi_clean(ivk);

    return jivk;
}