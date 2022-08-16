package it.airgap.sapling

public class Sapling {

    /******** Authorizing Key ********/

    @Throws(SaplingException::class)
    public fun getProofAuthorizingKey(spendingKey: ByteArray): ByteArray =
        extPakFromXsk(spendingKey) ?: throw SaplingException("Failed to derive proof authorizing key from spending key")

    private external fun extPakFromXsk(xsk: ByteArray): ByteArray?

    /******** Commitment ********/

    @Throws(SaplingException::class)
    public fun verifyCommitment(commitment: ByteArray, address: ByteArray, value: Long, rcm: ByteArray): Boolean {
        val cmu = extComputeCmu(address, value, rcm) ?: throw SaplingException("Failed to compute commitment")
        return commitment.contentEquals(cmu)
    }

    private external fun extComputeCmu(address: ByteArray, value: Long, rcm: ByteArray): ByteArray?

    /******** Init ********/

    @Throws(SaplingException::class)
    public fun initParameters(spendParameters: ByteArray, outputParameters: ByteArray) {
        if (!extInitParameters(spendParameters, outputParameters)) {
            throw SaplingException("Failed to initialize Sapling parameters")
        }
    }

    private external fun extInitParameters(spendParameters: ByteArray, outputParameters: ByteArray): Boolean

    /******** Key Agreement ********/

    @Throws(SaplingException::class)
    public fun keyAgreement(p: ByteArray, sk: ByteArray): ByteArray =
        extKeyAgreement(p, sk) ?: throw SaplingException("Failed to create key agreement")

    private external fun extKeyAgreement(p: ByteArray, sk: ByteArray): ByteArray?

    /******** Merkle Tree ********/

    @Throws(SaplingException::class)
    public fun merkleHash(depth: Long, lhs: ByteArray, rhs: ByteArray): ByteArray =
        extMerkleHash(depth, lhs, rhs) ?: throw SaplingException("Failed to create Merkle hash")

    private external fun extMerkleHash(depth: Long, lhs: ByteArray, rhs: ByteArray): ByteArray?

    /******** Nullifier ********/

    @Throws(SaplingException::class)
    public fun computeNullifier(viewingKey: ByteArray, address: ByteArray, value: Long, rcm: ByteArray, position: Long): ByteArray =
        extComputeNullifierWithXfvk(viewingKey, address, value, rcm, position) ?: throw SaplingException("Failed to compute nullifier")

    private external fun extComputeNullifierWithXfvk(
        xfvk: ByteArray,
        address: ByteArray,
        value: Long,
        rcm: ByteArray,
        position: Long,
    ): ByteArray?

    /******** Output Description ********/

    @Throws(SaplingException::class)
    public fun prepareOutputDescription(context: Long, viewingKey: ByteArray, address: ByteArray, rcm: ByteArray, value: Long): ByteArray =
        extOutputDescriptionFromXfvk(context, viewingKey, address, rcm, value) ?: throw SaplingException("Failed to prepare output description")

    @Throws(SaplingException::class)
    public fun preparePartialOutputDescription(context: Long, address: ByteArray, rcm: ByteArray, esk: ByteArray, value: Long): ByteArray =
        extPartialOutputDescription(context, address, rcm, esk, value) ?: throw SaplingException("Failed to prepare output description")

    private external fun extOutputDescriptionFromXfvk(
        context: Long,
        xfvk: ByteArray,
        address: ByteArray,
        rcm: ByteArray,
        value: Long,
    ): ByteArray?
    private external fun extOutputDescriptionFromXfvkWithMemo(
        context: Long,
        xfvk: ByteArray,
        address: ByteArray,
        rcm: ByteArray,
        value: Long,
        memo: ByteArray,
    ): ByteArray?
    private external fun extOutputDescriptionFromOvk(
        context: Long,
        ovk: ByteArray,
        address: ByteArray,
        rcm: ByteArray,
        value: Long,
    ): ByteArray?
    private external fun extPartialOutputDescription(context: Long, address: ByteArray, rcm: ByteArray, esk: ByteArray, value: Long): ByteArray?
    private external fun extDeriveEpkFromEsk(diversifier: ByteArray, esk: ByteArray): ByteArray?

    /******** Payment Address ********/

    @Throws(SaplingException::class)
    public fun getPaymentAddressFromViewingKey(viewingKey: ByteArray, index: ByteArray?): ByteArray =
        (if (index != null) extPaymentAddressFromXfvk(viewingKey, index)
        else extDefaultPaymentAddressFromXfvk(viewingKey))
            ?: throw SaplingException("Failed to create payment address")

    @Throws(SaplingException::class)
    public fun getNextPaymentAddressFromViewingKey(viewingKey: ByteArray, index: ByteArray): ByteArray =
        extNextPaymentAddressFromXfvk(viewingKey, index) ?: throw SaplingException("Failed to create next payment address")

    @Throws(SaplingException::class)
    public fun getRawPaymentAddressFromIncomingViewingKey(viewingKey: ByteArray, diversifier: ByteArray): ByteArray =
        extPaymentAddressFromIvk(viewingKey, diversifier) ?: throw SaplingException("Failed to create payment address")

    @Throws(SaplingException::class)
    public fun getDiversifierFromRawPaymentAddress(address: ByteArray): ByteArray =
        extDiversifierFromPaymentAddress(address) ?: throw SaplingException("Failed to extract diversifier from address")

    @Throws(SaplingException::class)
    public fun getPkdFromRawPaymentAddress(address: ByteArray): ByteArray =
        extPkdFromPaymentAddress(address) ?: throw SaplingException("Failed to extract pkd from address")

    private external fun extDefaultPaymentAddressFromXfvk(xfvk: ByteArray): ByteArray?
    private external fun extNextPaymentAddressFromXfvk(xfvk: ByteArray, index: ByteArray): ByteArray?
    private external fun extPaymentAddressFromXfvk(xfvk: ByteArray, index: ByteArray): ByteArray?
    private external fun extPaymentAddressFromIvk(ivk: ByteArray, diversifier: ByteArray): ByteArray?
    private external fun extDiversifierFromPaymentAddress(address: ByteArray): ByteArray?
    private external fun extPkdFromPaymentAddress(address: ByteArray): ByteArray?

    /******** Proving Context ********/

    public fun initProvingContext(): Long = extInitProvingContext()
    public fun dropProvingContext(context: Long) = extDropProvingContext(context)

    private external fun extInitProvingContext(): Long
    private external fun extDropProvingContext(context: Long)

    /******** Rand ********/

    @Throws(SaplingException::class)
    public fun randR(): ByteArray = extRandR() ?: throw SaplingException("Failed to create random scalar")

    private external fun extRandR(): ByteArray?

    /******** Signature ********/

    public fun createBindingSignature(context: Long, balance: Long, sighash: ByteArray): ByteArray =
        extCreateBindingSignature(context, balance, sighash) ?: throw SaplingException("Failed to create binding signature")

    private external fun extCreateBindingSignature(context: Long, balance: Long, sighash: ByteArray): ByteArray?

    /******** Spend Description ********/

    @Throws(SaplingException::class)
    public fun prepareSpendDescriptionWithSpendingKey(
        context: Long,
        spendingKey: ByteArray,
        address: ByteArray,
        rcm: ByteArray,
        ar: ByteArray,
        value: Long,
        anchor: ByteArray,
        merklePath: ByteArray,
    ): ByteArray = extSpendDescriptionFromXsk(context, spendingKey, address, rcm, ar, value, anchor, merklePath)
        ?: throw SaplingException("Failed to prepare spend description")

    @Throws(SaplingException::class)
    public fun prepareSpendDescriptionWithAuthorizingKey(
        context: Long,
        authorizingKey: ByteArray,
        address: ByteArray,
        rcm: ByteArray,
        ar: ByteArray,
        value: Long,
        anchor: ByteArray,
        merklePath: ByteArray,
    ): ByteArray = extSpendDescriptionFromPak(context, authorizingKey, address, rcm, ar, value, anchor, merklePath)
        ?: throw SaplingException("Failed to prepare spend description")

    @Throws(SaplingException::class)
    public fun signSpendDescription(spendDescription: ByteArray, spendingKey: ByteArray, ar: ByteArray, sighash: ByteArray): ByteArray =
        extSignSpendDescriptionWithXsk(spendDescription, spendingKey, ar, sighash) ?: throw SaplingException("Failed to sign spend description")

    private external fun extSpendDescriptionFromXsk(
        context: Long,
        xsk: ByteArray,
        address: ByteArray,
        rcm: ByteArray,
        ar: ByteArray,
        value: Long,
        anchor: ByteArray,
        merklePath: ByteArray,
    ): ByteArray?
    private external fun extSpendDescriptionFromPak(
        context: Long,
        pak: ByteArray,
        address: ByteArray,
        rcm: ByteArray,
        ar: ByteArray,
        value: Long,
        anchor: ByteArray,
        merklePath: ByteArray,
    ): ByteArray?
    private external fun extSignSpendDescriptionWithXsk(
        spendDescription: ByteArray,
        xsk: ByteArray,
        ar: ByteArray,
        sighash: ByteArray,
    ): ByteArray?

    /******** Spending Key ********/

    @Throws(SaplingException::class)
    public fun getExtendedSpendingKey(seed: ByteArray, derivationPath: String): ByteArray =
        extXsk(seed, derivationPath) ?: throw SaplingException("Failed to create extended spending key")

    private external fun extXsk(seed: ByteArray, derivationPath: String): ByteArray?

    /******** Viewing Key ********/

    @Throws(SaplingException::class)
    public fun getExtendedFullViewingKey(seed: ByteArray, derivationPath: String): ByteArray =
        extXfvk(seed, derivationPath) ?: throw SaplingException("Failed to create extended full viewing key")

    @Throws(SaplingException::class)
    public fun getExtendedFullViewingKeyFromSpendingKey(spendingKey: ByteArray): ByteArray =
        extXfvkFromXsk(spendingKey) ?: throw SaplingException("Failed to derive extended full viewing key from spending key")

    @Throws(SaplingException::class)
    public fun getOutgoingViewingKey(viewingKey: ByteArray): ByteArray =
        extOvkFromXfvk(viewingKey) ?: throw SaplingException("Failed to derive outgoing viewing key from extended full viewing key")

    @Throws(SaplingException::class)
    public fun getIncomingViewingKey(viewingKey: ByteArray): ByteArray =
        extXfvkToIvk(viewingKey) ?: throw SaplingException("Failed to derive incoming viewing key from extended full viewing key")

    private external fun extXfvk(seed: ByteArray, derivationPath: String): ByteArray?
    private external fun extXfvkFromXsk(xsk: ByteArray): ByteArray?
    private external fun extOvkFromXfvk(xfvk: ByteArray): ByteArray?
    private external fun extXfvkToIvk(xfvk: ByteArray): ByteArray?

    public companion object {
        init {
            System.loadLibrary("sapling")
        }
    }
}