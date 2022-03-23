#![no_std]

elrond_wasm_node::wasm_endpoints! {
    marketplace
    (
        putNftForSale
        buyNft
        withdrawNft
        makeOffer
        acceptOffer
        cancelOffer
        startAuction
        placeBid
        endAuction
        getVersion
        withdraw
        deposit
    )
}
