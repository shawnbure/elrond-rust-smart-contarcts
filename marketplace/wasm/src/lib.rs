////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    marketplace
    (
        acceptOffer
        blacklistCreator
        buyNft
        cancelOffer
        deposit
        endAuction
        getAssetMaxPrice
        getAssetMinPrice
        getAuction
        getCreatorLastWithdrawalEpoch
        getCreatorRoyalties
        getCreatorWithdrawalWaitingEpochs
        getDao
        getEgldDeposit
        getGlobalOpOngoing
        getNftSaleInfo
        getOffer
        getPlatformFeePercent
        getPlatformRoyalties
        getRemainingEpochsUntilClaim
        getRoyaltiesMaxFeePercent
        getStakingSCAddress
        getVersion
        isCreatorBlacklisted
        makeOffer
        placeBid
        putNftForSale
        removeCreatorFromBlacklist
        setAssetPriceRange
        setCreatorWithdrawalWaitingEpochs
        setPlatformFeePercent
        setRoyaltiesMaxFeePercent
        startAuction
        startGlobalOp
        stopGlobalOp
        withdraw
        withdrawCreatorRoyalties
        withdrawNft
        withdrawPlatformRoyalties
    )
}

elrond_wasm_node::wasm_empty_callback! {}
