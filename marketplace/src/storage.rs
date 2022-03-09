#![allow(clippy::too_many_arguments)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct NftId<M> 
where
    M: ManagedTypeApi,
{
    pub token_id: TokenIdentifier<M>,
    pub nonce: u64,
}

//impl <M> ManagedType<M> for NftId<M>
impl<M: ManagedTypeApi> NftId<M>
where
    M: ManagedTypeApi,
{
    fn new(token_id: TokenIdentifier, nonce: u64) -> Self {
        NftId { 
            token_id, 
            nonce 
        }
    }
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct NftSaleInfo<M> 
where
    M: ManagedTypeApi,
{
    pub owner: ManagedAddress<M>,
    pub price: BigUint<M>,
    pub timestamp: u64,
}

impl<M: ManagedTypeApi> NftSaleInfo<M> 
where
    M: ManagedTypeApi,
{
    pub fn new(owner: ManagedAddress, price: BigUint, timestamp: u64) -> Self {
        NftSaleInfo {
            owner,
            price,
            timestamp,
        }
    }
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct AuctionInfo<M> 
where
    M: ManagedTypeApi,
{
    pub owner: ManagedAddress<M>,
    pub min_bid: BigUint<M>,
    pub start_time: u64,
    pub deadline: u64,
    pub created_at: u64,
    pub highest_bidder: ManagedAddress<M>,
    pub bid: BigUint<M>,
}

impl<M: ManagedTypeApi> AuctionInfo<M> 
where
    M: ManagedTypeApi,
{
    pub fn new(
        owner: ManagedAddress<M>,
        min_bid: BigUint<M>,
        start_time: u64,
        deadline: u64,
        created_at: u64,
        highest_bidder: ManagedAddress<M>,
        bid: BigUint<M>,
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
pub struct Offer<M> 
where
    M: ManagedTypeApi,
{
    pub amount: BigUint<M>,
    pub expire: u64,
}

impl<M: ManagedTypeApi> Offer<M> 
where
    M: ManagedTypeApi,
{
    pub fn new(amount: BigUint, expire: u64) -> Self {
        Offer { amount, expire }
    }
}

impl<M: ManagedTypeApi> Default for Offer<M> {
    fn default() -> Self {
        Offer {
            amount: BigUint::zero(),
            expire: 0,
        }
    }
}


#[elrond_wasm_derive::module]
//#[elrond_wasm::module]
pub trait StorageModule {

    #[view(getPlatformFeePercent)]
    #[storage_mapper("platform_fee_percent")]
    fn platform_fee_percent(&self) -> SingleValueMapper<Self::Api, u64>;

    #[view(getAssetMinPrice)]
    #[storage_mapper("asset_min_price")]
    fn asset_min_price(&self) -> SingleValueMapper<Self::Api, BigUint>;

    #[view(getAssetMaxPrice)]
    #[storage_mapper("asset_max_price")]
    fn asset_max_price(&self) -> SingleValueMapper<Self::Api, BigUint>;

    #[view(getRoyaltiesMaxFeePercent)]
    #[storage_mapper("royalties_max_fee_percent")]
    fn royalties_max_fee_percent(&self) -> SingleValueMapper<Self::Api, u64>;

    #[view(getCreatorWithdrawalWaitingEpochs)]
    #[storage_mapper("creator_withdrawal_waiting_epochs")]
    fn creator_withdrawal_waiting_epochs(&self) -> SingleValueMapper<Self::Api, u64>;

    #[view(isCreatorBlacklisted)]
    #[storage_mapper("creator_blacklist")]
    fn creator_blacklist(&self, address: &Address) -> SingleValueMapper<Self::Api, bool>;

    #[view(getEgldDeposit)]
    #[storage_mapper("egld_deposit")]
    fn egld_deposit(&self, address: &Address) -> SingleValueMapper<Self::Api, BigUint>;

    #[view(getCreatorRoyalties)]
    #[storage_mapper("creator_royalties")]
    fn creator_royalties(
        &self,
        address: &Address,
    ) -> SingleValueMapper<Self::Api, BigUint>;

    #[view(getCreatorLastWithdrawalEpoch)]
    #[storage_mapper("creator_last_withdrawal_epoch")]
    fn creator_last_withdrawal_epoch(
        &self,
        address: &Address,
    ) -> SingleValueMapper<Self::Api, u64>;

    #[view(getPlatformRoyalties)]
    #[storage_mapper("platform_royalties")]
    fn platform_royalties(&self) -> SingleValueMapper<Self::Api, BigUint>;

    #[view(getNftSaleInfo)]
    #[storage_mapper("nft_sale_info")]
    fn nft_sale_info(
        &self,
        nft_id: &NftId,
    ) -> SingleValueMapper<Self::Api, NftSaleInfo<BigUint>>;

    #[view(getOffer)]
    #[storage_mapper("offers")]
    fn offers(
        &self,
        caller: &Address,
        nft_id: &NftId,
        nft_list_timestamp: u64,
    ) -> SingleValueMapper<Self::Api, Offer<BigUint>>;

    #[view(getAuction)]
    #[storage_mapper("auction")]
    fn auction(
        &self,
        nft_id: &NftId,
    ) -> SingleValueMapper<Self::Api, AuctionInfo<BigUint>>;
}
