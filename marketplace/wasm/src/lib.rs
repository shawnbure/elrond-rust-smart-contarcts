#![no_std]

elrond_wasm_node::wasm_endpoints! {
    marketplace
    (
        putNftForSale
        buyNft
        withdrawNft
        makeOffer
        cancelOffer
        startAuction
        placeBid
        endAuction
    )
}
