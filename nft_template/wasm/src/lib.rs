////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    nft_template
    (
        allowMintingThroughMarketplace
        changeBuyerBuyLimit
        createBuyerAddress
        denyMintingThroughMarketplace
        getAdminPubKey
        getBuyCount
        getBuyLimit
        getBuyerWhiteListCheck
        getImageBaseUri
        getImageExtension
        getLeftForSale
        getMarketplaceAdmin
        getMarketplaceBalance
        getMaxSupply
        getMaxSupplyAndTotalSold
        getMetadataBaseUri
        getMetadataExtension
        getPrice
        getRoyalties
        getSaleStart
        getTokenId
        getTokenNameBase
        getTotalSold
        giveaway
        isMintingThroughMarketplaceDenied
        isSalePaused
        marketplaceWithdraw
        mintTokens
        mintTokensThroughMarketplace
        pauseSale
        requestWithdraw
        resumeSale
        setPrice
        shuffle
        updateBuyerWhitelistCheck
        updateMetadataExtension
        updateSaleStart
        withdraw
    )
}

elrond_wasm_node::wasm_empty_callback! {}
