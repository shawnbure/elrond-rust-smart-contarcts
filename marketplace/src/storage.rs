elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct NftId {
    pub token_id: TokenIdentifier,
    pub nonce: u64,
}

impl NftId {
    pub fn new(token_id: TokenIdentifier, nonce: u64) -> Self {
        NftId { token_id, nonce }
    }
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct NftSaleInfo<BigUint: BigUintApi> {
    pub owner: Address,
    pub uri: BoxedBytes,
    pub price: BigUint,
    pub platform_fee: u64,
    pub timestamp: u64,
}

impl<BigUint: BigUintApi> NftSaleInfo<BigUint> {
    pub fn new(
        owner: Address,
        uri: BoxedBytes,
        price: BigUint,
        platform_fee: u64,
        timestamp: u64,
    ) -> Self {
        NftSaleInfo {
            owner,
            uri,
            price,
            platform_fee,
            timestamp,
        }
    }
}
#[elrond_wasm::module]
pub trait StorageModule {
    #[view(getPlatformFeePercent)]
    #[storage_mapper("platform_fee_percent")]
    fn platform_fee_percent(&self) -> SingleValueMapper<Self::Storage, u64>;

    #[storage_mapper("nft_sale_info")]
    fn nft_sale_info(
        &self,
        nft_id: &NftId,
    ) -> SingleValueMapper<Self::Storage, NftSaleInfo<Self::BigUint>>;
}
