#![no_std]

elrond_wasm_node::wasm_endpoints! {
    deployer
    (
        deployNFTTemplateContract
        changeOwner
        withdraw
        setMarketplaceAddress
        setNftTemplateAddress
        getMarketplaceAddress
        getNftTemplateAddress
        getOwnerOfContract
    )
}
