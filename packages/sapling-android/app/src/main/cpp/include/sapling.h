#ifndef AIRGAP_SAPLING_H
#define AIRGAP_SAPLING_H

#include <jni.h>
#include "stdbool.h"
#include <vector>

#include "utils.h"

#ifdef __cplusplus
extern "C" {
#endif
    /******** Commitment ********/

    size_t c_compute_cmu(
            const unsigned char *address,
            size_t address_len,
            uint64_t value,
            const unsigned char *rcm,
            size_t rcm_len,
            const unsigned char *&cmu_res
    );

    /******** Init ********/

    bool c_init_params(
            const unsigned char *spend_params,
            size_t spend_params_len,
            const unsigned char *output_params,
            size_t output_params_len
    );

    /******** Key Agreement ********/

    size_t c_key_agreement(
            const unsigned char *p,
            size_t p_len,
            const unsigned char *sk,
            size_t sk_len,
            const unsigned char *&ka_res
    );

    /******** Merkle Tree ********/

    size_t c_merkle_hash(
            uint64_t depth,
            const unsigned char *lhs,
            size_t lhs_len,
            const unsigned char *rhs,
            size_t rhs_len,
            const unsigned char *&merkle_hash_res
    );

    /******** Nullifier ********/

    size_t c_compute_nullifier_with_xfvk(
            const unsigned char *xfvk,
            size_t xfvk_len,
            const unsigned char *address,
            size_t address_len,
            uint64_t value,
            const unsigned char *rcm,
            size_t rcm_len,
            uint64_t position,
            const unsigned char *&nullifier_res
    );

    /******** Output Description ********/

    size_t c_output_description_from_xfvk(
            const void *ctx,
            const unsigned char *xfvk,
            size_t xfvk_len,
            const unsigned char *to,
            size_t to_len,
            const unsigned char *rcm,
            size_t rcm_len,
            uint64_t value,
            const unsigned char *&description_res
    );

    size_t c_output_description_from_xfvk_with_memo(
            const void *ctx,
            const unsigned char *xfvk,
            size_t xfvk_len,
            const unsigned char *to,
            size_t to_len,
            const unsigned char *rcm,
            size_t rcm_len,
            uint64_t value,
            const unsigned char *memo,
            size_t memo_len,
            const unsigned char *&description_res
    );

    size_t c_output_description_from_ovk(
            const void *ctx,
            const unsigned char *ovk,
            size_t ovk_len,
            const unsigned char *to,
            size_t to_len,
            const unsigned char *rcm,
            size_t rcm_len,
            uint64_t value,
            const unsigned char *&description_res
    );

    size_t c_partial_output_description(
            const void *ctx,
            const unsigned char *to,
            size_t to_len,
            const unsigned char *rcm,
            size_t rcm_len,
            const unsigned char *esk,
            size_t esk_len,
            uint64_t value,
            const unsigned char *&description_res
    );

    size_t c_derive_epk_from_esk(
            const unsigned char *diversifier,
            size_t diversifier_len,
            const unsigned char *esk,
            size_t esk_len,
            const unsigned char *&epk_res
    );

    /******** Payment Address ********/

    size_t c_default_payment_address_from_xfvk(
            const unsigned char *xfvk,
            size_t xfvk_len,
            const unsigned char *&xfvk_address_res
    );

    size_t c_next_payment_address_from_xfvk(
            const unsigned char *xfvk,
            size_t xfvk_len,
            const unsigned char *index,
            size_t index_len,
            const unsigned char *&xfvk_address_res
    );

    size_t c_payment_address_from_xfvk(
            const unsigned char *xfvk,
            size_t xfvk_len,
            const unsigned char *index,
            size_t index_len,
            const unsigned char *&xfvk_address_res
    );

    size_t c_payment_address_from_ivk(
            const unsigned char *ivk,
            size_t ivk_len,
            const unsigned char *diversifier,
            size_t diversifier_len,
            const unsigned char *&xfvk_address_res
    );

    size_t c_diversifier_from_payment_address(
            const unsigned char *address,
            size_t address_len,
            const unsigned char *&diversifier_res
    );

    size_t c_pkd_from_payment_address(
            const unsigned char *address,
            size_t address_len,
            const unsigned char *&pkd_res
    );

    /******** Proving Context ********/

    void *c_init_proving_context();
    void c_drop_proving_context(void *ctx);

    /******** Rand ********/

    size_t c_rand_r(const unsigned char *&r_res);

    /******** Signature ********/

    size_t c_binding_signature(
            void *ctx,
            int64_t value_balance,
            const unsigned char *sighash,
            size_t sighash_len,
            const unsigned char *&signature_res
    );

    /******** Spend Description ********/

    size_t c_spend_description_from_xsk(
            void *ctx,
            const unsigned char *xsk,
            size_t xsk_len,
            const unsigned char *address,
            size_t address_len,
            const unsigned char *rcm,
            size_t rcm_len,
            const unsigned char *ar,
            size_t ar_len,
            uint64_t value,
            const unsigned char *anchor,
            size_t anchor_len,
            const unsigned char *merkle_path,
            size_t merkle_path_len,
            const unsigned char *&description_res
    );

    size_t c_sign_spend_description_with_xsk(
            const unsigned char *spend_description,
            size_t spend_description_len,
            const unsigned char *xsk,
            size_t xsk_len,
            const unsigned char *ar,
            size_t ar_len,
            const unsigned char *sighash,
            size_t sighash_len,
            const unsigned char *&description_res
    );

    /******** Spending Key ********/

    size_t c_xsk(const unsigned char *seed, size_t seed_len, const char *derivation_path, const unsigned char *&xsk_res);

    /******** Viewing Key ********/

    size_t c_xfvk(const unsigned char *seed, size_t seed_len, const char *derivation_path, const unsigned char *&xfvk_res);
    size_t c_xfvk_from_xsk(const unsigned char *xsk, size_t xsk_len, const unsigned char *&xfvk_res);
    size_t c_ovk_from_xfvk(const unsigned char *xfvk, size_t xfvk_len, const unsigned char *&ovk_res);
    size_t c_xfvk_to_ivk(const unsigned char *xfvk, size_t xfvk_len, const unsigned char *&ivk_res);
#ifdef __cplusplus
};
#endif

#endif // AIRGAP_SAPLING_H