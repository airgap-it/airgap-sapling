#ifndef AIRGAP_SAPLING_H
#define AIRGAP_SAPLING_H

#include "stdlib.h"
#include "stddef.h"
#include "stdbool.h"

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus
    /******** Authorizing Key ********/

    unsigned char *c_pak_from_xsk(const unsigned char *xsk, size_t xsk_len, size_t *pak_len)

    /******** Commitment ********/

    unsigned char *c_compute_cmu(
            const unsigned char *address,
            size_t address_len,
            uint64_t value,
            const unsigned char *rcm,
            size_t rcm_len,
            size_t *cmu_len
    );

    /******** Init ********/

    bool c_init_params(
            const unsigned char *spend_params,
            size_t spend_params_len,
            const unsigned char *output_params,
            size_t output_params_len
    );

    /******** Key Agreement ********/

    unsigned char *c_key_agreement(
            const unsigned char *p,
            size_t p_len,
            const unsigned char *sk,
            size_t sk_len,
            size_t *ka_len
    );

    /******** Merkle Tree ********/

    unsigned char *c_merkle_hash(
            size_t depth,
            const unsigned char *lhs,
            size_t lhs_len,
            const unsigned char *rhs,
            size_t rhs_len,
            size_t *merkle_hash_len
    );

    /******** Nullifier ********/

    unsigned char *c_compute_nullifier_with_xfvk(
            const unsigned char *xfvk,
            size_t xfvk_len,
            const unsigned char *address,
            size_t address_len,
            uint64_t value,
            const unsigned char *rcm,
            size_t rcm_len,
            uint64_t position,
            size_t *nullifier_len
    );

    /******** Output Description ********/

    unsigned char *c_output_description_from_xfvk(
            void *ctx,
            const unsigned char *xfvk,
            size_t xfvk_len,
            const unsigned char *to,
            size_t to_len,
            const unsigned char *rcm,
            size_t rcm_len,
            uint64_t value,
            size_t *description_len
    );

    unsigned char *c_output_description_from_xfvk_with_memo(
            void *ctx,
            const unsigned char *xfvk,
            size_t xfvk_len,
            const unsigned char *to,
            size_t to_len,
            const unsigned char *rcm,
            size_t rcm_len,
            uint64_t value,
            const unsigned char *memo,
            size_t memo_len,
            size_t *description_len
    );

    unsigned char *c_output_description_from_ovk(
            void *ctx,
            const unsigned char *ovk,
            size_t ovk_len,
            const unsigned char *to,
            size_t to_len,
            const unsigned char *rcm,
            size_t rcm_len,
            uint64_t value,
            size_t *description_len
    );

    unsigned char *c_partial_output_description(
            void *ctx,
            const unsigned char *to,
            size_t to_len,
            const unsigned char *rcm,
            size_t rcm_len,
            const unsigned char *esk,
            size_t esk_len,
            uint64_t value,
            size_t *description_len
    );

    unsigned char *c_derive_epk_from_esk(
            const unsigned char *diversifier,
            size_t diversifier_len,
            const unsigned char *esk,
            size_t esk_len,
            size_t *epk_len
    );

    /******** Payment Address ********/

    unsigned char *c_default_payment_address_from_xfvk(
            const unsigned char *xfvk,
            size_t xfvk_len,
            size_t *xfvk_address_res
    );

    unsigned char *c_next_payment_address_from_xfvk(
            const unsigned char *xfvk,
            size_t xfvk_len,
            const unsigned char *index,
            size_t index_len,
            size_t *address_len
    );

    unsigned char *c_payment_address_from_xfvk(
            const unsigned char *xfvk,
            size_t xfvk_len,
            const unsigned char *index,
            size_t index_len,
            size_t *address_len
    );

    unsigned char *c_payment_address_from_ivk(
            const unsigned char *ivk,
            size_t ivk_len,
            const unsigned char *diversifier,
            size_t diversifier_len,
            size_t *address_len
    );

    unsigned char *c_diversifier_from_payment_address(
            const unsigned char *address,
            size_t address_len,
            size_t *diversifier_len
    );

    unsigned char *c_pkd_from_payment_address(
            const unsigned char *address,
            size_t address_len,
            size_t *pkd_len
    );

    /******** Proving Context ********/

    void *c_init_proving_context();
    void c_drop_proving_context(void *ctx);

    /******** Rand ********/

    unsigned char *c_rand_r(size_t *r_len);

    /******** Signature ********/

    unsigned char *c_binding_signature(
            void *ctx,
            int64_t value_balance,
            const unsigned char *sighash,
            size_t sighash_len,
            size_t *signature_len
    );

    /******** Spend Description ********/

    unsigned char *c_spend_description_from_xsk(
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
            size_t *description_len
    );

    unsigned char *c_spend_description_from_pak(
            void *ctx,
            const unsigned char *pak,
            size_t pak_len,
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
            size_t *description_len
    );

    unsigned char *c_sign_spend_description_with_xsk(
            const unsigned char *spend_description,
            size_t spend_description_len,
            const unsigned char *xsk,
            size_t xsk_len,
            const unsigned char *ar,
            size_t ar_len,
            const unsigned char *sighash,
            size_t sighash_len,
            size_t *description_len
    );

    /******** Spending Key ********/

    unsigned char *c_xsk(const unsigned char *seed, size_t seed_len, const char *derivation_path, size_t *xsk_len);

    /******** Viewing Key ********/

    unsigned char *c_xfvk(const unsigned char *seed, size_t seed_len, const char *derivation_path, size_t *xfvk_len);
    unsigned char *c_xfvk_from_xsk(const unsigned char *xsk, size_t xsk_len, size_t *xfvk_len);
    unsigned char *c_ovk_from_xfvk(const unsigned char *xfvk, size_t xfvk_len, size_t *ovk_len);
    unsigned char *c_xfvk_to_ivk(const unsigned char *xfvk, size_t xfvk_len, size_t *ivk_len);
#ifdef __cplusplus
};
#endif // __cplusplus

#endif // AIRGAP_SAPLING_H