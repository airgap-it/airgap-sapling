import SaplingFFI

public struct Sapling {
    
    public init() {}
    
    // MARK: Authorizing Key

    public func getProofAuthorizingKey(from spendingKey: ExtendedSpendingKey) throws -> ProofAuthorizingKey {
        var keyCount = 0
        guard let key = c_pak_from_xsk(spendingKey, spendingKey.count, &keyCount)?.toArray(count: keyCount) else {
            throw Error.deriveKeyFailed
        }

        return key
    }

    // MARK: Commitment

    public func verifyCommitment(_ commitment: [UInt8], to address: Address, forValue value: UInt64, with rcm: Rcm) throws -> Bool {
        var cmuCount = 0
        guard let cmu = c_compute_cmu(address, address.count, value, rcm, rcm.count, &cmuCount)?.toArray(count: cmuCount) else {
            throw Error.computeCommitmentFailed
        }

        return commitment == cmu
    }

    // MARK: Init

    public func initParameters(spend spendParameters: [UInt8], output outputParameters: [UInt8]) throws {
        guard c_init_params(spendParameters, spendParameters.count, outputParameters, outputParameters.count) else {
            throw Error.initParametersFailed
        }
    }

    // MARK: Key Agreement

    public func keyAgreement(p: [UInt8], sk: [UInt8]) throws -> [UInt8] {
        var kaCount = 0
        guard let ka = c_key_agreement(p, p.count, sk, sk.count, &kaCount)?.toArray(count: kaCount) else {
            throw Error.createKeyAgreementFailed
        }

        return ka
    }

    // MARK: Merkle Tree

    public func merkleHash(ofDepth depth: Int, lhs: [UInt8], rhs: [UInt8]) throws -> [UInt8] {
        var merkleHashCount = 0
        guard let merkleHash = c_merkle_hash(depth, lhs, lhs.count, rhs, rhs.count, &merkleHashCount)?.toArray(count: merkleHashCount) else {
            throw Error.createMerkleHashFailed
        }

        return merkleHash
    }

    // MARK: Nullifier

    public func computeNullifier(using viewingKey: ExtendedFullViewingKey, to address: Address, forValue value: UInt64, with rcm: Rcm, at position: UInt64) throws -> [UInt8] {
        var nullifierCount = 0
        guard let nullifier = c_compute_nullifier_with_xfvk(
                viewingKey, viewingKey.count,
                address, address.count,
                value,
                rcm, rcm.count,
                position,
                &nullifierCount
        )?.toArray(count: nullifierCount) else {
            throw Error.computeNullifierFailed
        }

        return nullifier
    }

    // MARK: Output Description

    public func prepareOutputDescription(
        with context: Context,
        using viewingKey: ExtendedFullViewingKey,
        to address: Address,
        withRcm rcm: Rcm,
        ofValue value: UInt64
    ) throws -> [UInt8] {
        var descriptionCount = 0
        guard let description = c_output_description_from_xfvk(context, viewingKey, viewingKey.count, address, address.count, rcm, rcm.count, value, &descriptionCount)?.toArray(count: descriptionCount) else {
            throw Error.outputDescriptionFailed
        }

        return description
    }

    public func preparePartialOutputDescription(
        with context: Context,
        to address: Address,
        withRcm rcm: Rcm,
        withEsk esk: Esk,
        ofValue value: UInt64
    ) throws -> [UInt8] {
        var descriptionCount = 0
        guard let description = c_partial_output_description(
                context,
                address, address.count,
                rcm, rcm.count,
                esk, esk.count,
                value,
                &descriptionCount
        )?.toArray(count: descriptionCount) else {
            throw Error.outputDescriptionFailed
        }

        return description
    }

    // MARK: Payment Address

    public func getPaymentAddress(from viewingKey: ExtendedFullViewingKey, at index: Index) throws -> [UInt8] {
        var addressCount = 0
        guard let address = c_payment_address_from_xfvk(viewingKey, viewingKey.count, index, index.count, &addressCount)?.toArray(count: addressCount) else {
            throw Error.createAddressFailed
        }

        return address
    }

    public func getNextPaymentAddress(from viewingKey: ExtendedFullViewingKey, lastAt index: Index) throws -> [UInt8] {
        var addressCount = 0
        guard let address = c_next_payment_address_from_xfvk(viewingKey, viewingKey.count, index, index.count, &addressCount)?.toArray(count: addressCount) else {
            throw Error.createAddressFailed
        }

        return address
    }

    public func getRawPaymentAddress(from incomingViewingKey: IncomingViewingKey, with diversifier: Diversifier) throws -> [UInt8] {
        var addressCount = 0
        guard let address = c_payment_address_from_ivk(
                incomingViewingKey, incomingViewingKey.count,
                diversifier, diversifier.count,
                &addressCount
        )?.toArray(count: addressCount) else {
            throw Error.createAddressFailed
        }

        return address
    }

    public func getDiversifier(fromRaw rawAddress: Address) throws -> [UInt8] {
        var diversifierCount = 0
        guard let diversifier = c_diversifier_from_payment_address(rawAddress, rawAddress.count, &diversifierCount)?.toArray(count: diversifierCount) else {
            throw Error.extractFromAddressFailed
        }

        return diversifier
    }

    public func getPkd(fromRaw rawAddress: Address) throws -> [UInt8] {
        var pkdCount = 0
        guard let pkd = c_pkd_from_payment_address(rawAddress, rawAddress.count, &pkdCount)?.toArray(count: pkdCount) else {
            throw Error.extractFromAddressFailed
        }

        return pkd
    }

    // MARK: Proving Context

    public func initProvingContext() throws -> Context {
        guard let context = c_init_proving_context() else {
            throw Error.initProvingContextFailed
        }

        return context
    }

    public func dropProvingContext(_ context: Context) {
        c_drop_proving_context(context)
    }

    // MARK: Rand

    public func randR() throws -> [UInt8] {
        var rCount = 0
        guard let r = c_rand_r(&rCount)?.toArray(count: rCount) else {
            throw Error.createRandomScalarFailed
        }

        return r
    }

    // MARK: Signature

    public func createBindingSignature(with context: Context, balance: Int64, sighash: [UInt8]) throws -> [UInt8] {
        var signatureCount = 0
        guard let signature = c_binding_signature(context, balance, sighash, sighash.count, &signatureCount)?.toArray(count: signatureCount) else {
            throw Error.createBindingSignatureFailed
        }

        return signature
    }

    // MARK: Spend Description

    public func prepareSpendDescriptionWithSpendingKey(
        with context: Context,
        using spendingKey: ExtendedSpendingKey,
        to address: Address,
        withRcm rcm: Rcm,
        withAr ar: Ar,
        ofValue value: UInt64,
        withAnchor anchor: Anchor,
        at merklePath: MerklePath
    ) throws -> [UInt8] {
        var descriptionCount = 0
        guard let description = c_spend_description_from_xsk(
                context,
                spendingKey, spendingKey.count,
                address, address.count,
                rcm, rcm.count,
                ar, ar.count,
                value,
                anchor, anchor.count,
                merklePath, merklePath.count,
                &descriptionCount
        )?.toArray(count: descriptionCount) else {
            throw Error.spendDescriptionFailed
        }

        return description
    }

    public func prepareSpendDescriptionWithAuthorizingKey(
        with context: Context,
        using authorizingKey: ProofAuthorizingKey,
        to address: Address,
        withRcm rcm: Rcm,
        withAr ar: Ar,
        ofValue value: UInt64,
        withAnchor anchor: Anchor,
        at merklePath: MerklePath
    ) throws -> [UInt8] {
        var descriptionCount = 0
        guard let description = c_spend_description_from_pak(
                context,
                authorizingKey, authorizingKey.count,
                address, address.count,
                rcm, rcm.count,
                ar, ar.count,
                value,
                anchor, anchor.count,
                merklePath, merklePath.count,
                &descriptionCount
        )?.toArray(count: descriptionCount) else {
            throw Error.spendDescriptionFailed
        }

        return description
    }

    public func signSpendDescription(_ spendDescription: [UInt8], using spendingKey: ExtendedSpendingKey, with ar: Ar, sighash: [UInt8]) throws -> [UInt8] {
        var signedDescriptionCount = 0
        guard let signedDescription = c_sign_spend_description_with_xsk(
                spendDescription, spendDescription.count,
                spendingKey, spendingKey.count,
                ar, ar.count,
                sighash, sighash.count,
                &signedDescriptionCount
        )?.toArray(count: signedDescriptionCount) else {
            throw Error.spendDescriptionFailed
        }

        return signedDescription
    }

    // MARK: Spending Key

    public func getExtendedSpendingKey(from seed: Seed, derivationPath: String) throws -> ExtendedSpendingKey {
        var keyCount = 0
        return try derivationPath.withCString { derivationPath in
            guard let key = c_xsk(seed, seed.count, derivationPath, &keyCount)?.toArray(count: keyCount) else {
                throw Error.deriveKeyFailed
            }

            return key
        }
    }

    // MARK: Viewing Key

    public func getExtendedFullViewingKey(from seed: Seed, derivationPath: String) throws -> ExtendedFullViewingKey {
        var keyCount = 0
        return try derivationPath.withCString { derivationPath in
            guard let key = c_xfvk(seed, seed.count, derivationPath, &keyCount)?.toArray(count: keyCount) else {
                throw Error.deriveKeyFailed
            }

            return key
        }
    }

    public func getExtendedFullViewingKey(from spendingKey: ExtendedSpendingKey) throws -> ExtendedFullViewingKey {
        var keyCount = 0
        guard let key = c_xfvk_from_xsk(spendingKey, spendingKey.count, &keyCount)?.toArray(count: keyCount) else {
            throw Error.deriveKeyFailed
        }

        return key
    }

    public func getOutgoingViewingKey(from viewingKey: ExtendedFullViewingKey) throws -> OutgoingViewingKey {
        var keyCount = 0
        guard let key = c_ovk_from_xfvk(viewingKey, viewingKey.count, &keyCount)?.toArray(count: keyCount) else {
            throw Error.deriveKeyFailed
        }

        return key
    }

    public func getIncomingViewingKey(from viewingKey: ExtendedFullViewingKey) throws -> IncomingViewingKey {
        var keyCount = 0
        guard let key = c_xfvk_to_ivk(viewingKey, viewingKey.count, &keyCount)?.toArray(count: keyCount) else {
            throw Error.deriveKeyFailed
        }

        return key
    }
    
    public typealias Context = UnsafeMutableRawPointer
    
    public typealias Seed = [UInt8]
    public typealias ExtendedSpendingKey = [UInt8]
    public typealias ExtendedFullViewingKey = [UInt8]
    public typealias IncomingViewingKey = [UInt8]
    public typealias OutgoingViewingKey = [UInt8]
    public typealias ProofAuthorizingKey = [UInt8]
    
    public typealias Address = [UInt8]
    public typealias Diversifier = [UInt8]
    public typealias Index = [UInt8]
    
    public typealias Rcm = [UInt8]
    public typealias Esk = [UInt8]
    public typealias Ar = [UInt8]
    public typealias Anchor = [UInt8]
    public typealias MerklePath = [UInt8]
 
    public enum Error: Swift.Error {
        case computeCommitmentFailed
        case initParametersFailed
        case createKeyAgreementFailed
        case createMerkleHashFailed
        case computeNullifierFailed
        case outputDescriptionFailed
        case createAddressFailed
        case extractFromAddressFailed
        case initProvingContextFailed
        case createRandomScalarFailed
        case createBindingSignatureFailed
        case spendDescriptionFailed
        case deriveKeyFailed
    }
}

extension UnsafeMutablePointer where Pointee == UInt8 {
    func toArray(count: Int) -> [UInt8] {
        let bytes = Array(UnsafeBufferPointer(start: self, count: count))
        deinitialize(count: count)
        deallocate()
        
        return bytes
    }
}
