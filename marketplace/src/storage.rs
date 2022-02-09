#![allow(clippy::too_many_arguments)]

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
    pub price: BigUint,
    pub timestamp: u64,
}

impl<BigUint: BigUintApi> NftSaleInfo<BigUint> {
    pub fn new(owner: Address, price: BigUint, timestamp: u64) -> Self {
        NftSaleInfo {
            owner,
            price,
            timestamp,
        }
    }
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct AuctionInfo<BigUint: BigUintApi> {
    pub owner: Address,
    pub min_bid: BigUint,
    pub start_time: u64,
    pub deadline: u64,
    pub created_at: u64,
    pub highest_bidder: Address,
    pub bid: BigUint,
}

impl<BigUint: BigUintApi> AuctionInfo<BigUint> {
    pub fn new(
        owner: Address,
        min_bid: BigUint,
        start_time: u64,
        deadline: u64,
        created_at: u64,
        highest_bidder: Address,
        bid: BigUint,
    ) -> Self {
        AuctionInfo {
            owner,
            min_bid,
            start_time,
            deadline,
            created_at,
            highest_bidder,
            bid,
        }
    }
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct Offer<BigUint: BigUintApi> {
    pub amount: BigUint,
    pub expire: u64,
}

impl<BigUint: BigUintApi> Offer<BigUint> {
    pub fn new(amount: BigUint, expire: u64) -> Self {
        Offer { amount, expire }
    }
}

impl<BigUint: BigUintApi> Default for Offer<BigUint> {
    fn default() -> Self {
        Offer {
            amount: BigUint::zero(),
            expire: 0,
        }
    }
}

#[elrond_wasm::module]
pub trait StorageModule {
    #[view(getPlatformFeePercent)]
    #[storage_mapper("platform_fee_percent")]
    fn platform_fee_percent(&self) -> SingleValueMapper<Self::Storage, u64>;

    #[view(getAssetMinPrice)]
    #[storage_mapper("asset_min_price")]
    fn asset_min_price(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[view(getAssetMaxPrice)]
    #[storage_mapper("asset_max_price")]
    fn asset_max_price(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[view(getRoyaltiesMaxFeePercent)]
    #[storage_mapper("royalties_max_fee_percent")]
    fn royalties_max_fee_percent(&self) -> SingleValueMapper<Self::Storage, u64>;

    #[view(getCreatorWithdrawalWaitingEpochs)]
    #[storage_mapper("creator_withdrawal_waiting_epochs")]
    fn creator_withdrawal_waiting_epochs(&self) -> SingleValueMapper<Self::Storage, u64>;

    #[view(isCreatorBlacklisted)]
    #[storage_mapper("creator_blacklist")]
    fn creator_blacklist(&self, address: &Address) -> SingleValueMapper<Self::Storage, bool>;

    #[view(getEgldDeposit)]
    #[storage_mapper("egld_deposit")]
    fn egld_deposit(&self, address: &Address) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[view(getCreatorRoyalties)]
    #[storage_mapper("creator_royalties")]
    fn creator_royalties(
        &self,
        address: &Address,
    ) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[view(getCreatorLastWithdrawalEpoch)]
    #[storage_mapper("creator_last_withdrawal_epoch")]
    fn creator_last_withdrawal_epoch(
        &self,
        address: &Address,
    ) -> SingleValueMapper<Self::Storage, u64>;

    #[view(getPlatformRoyalties)]
    #[storage_mapper("platform_royalties")]
    fn platform_royalties(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[view(getNftSaleInfo)]
    #[storage_mapper("nft_sale_info")]
    fn nft_sale_info(
        &self,
        nft_id: &NftId,
    ) -> SingleValueMapper<Self::Storage, NftSaleInfo<Self::BigUint>>;

    #[view(getOffer)]
    #[storage_mapper("offers")]
    fn offers(
        &self,
        caller: &Address,
        nft_id: &NftId,
        nft_list_timestamp: u64,
    ) -> SingleValueMapper<Self::Storage, Offer<Self::BigUint>>;

    #[view(getAuction)]
    #[storage_mapper("auction")]
    fn auction(
        &self,
        nft_id: &NftId,
    ) -> SingleValueMapper<Self::Storage, AuctionInfo<Self::BigUint>>;

    #[view(getAdmin)]
    #[storage_mapper("admin_pub")]
    fn admin_pub(&self)  -> SingleValueMapper<Self::Storage, BoxedBytes>;
}
