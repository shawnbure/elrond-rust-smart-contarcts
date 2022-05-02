#![allow(clippy::too_many_arguments)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();






#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct StakedPool<M:ManagedTypeApi>
{  
    pub arrayStakedAddressNFTs: Vec<StakedAddressNFTs<M>>
}


impl<M: ManagedTypeApi> StakedPool<M>
where
    M: ManagedTypeApi,
{
    pub fn new(arrayStakedAddressNFTs: Vec<StakedAddressNFTs<M>>
    ) -> Self {
        StakedPool {
            arrayStakedAddressNFTs
        }
    }
}



#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct StakedAddressNFTs<M>
where
    M: ManagedTypeApi,
{
    pub address: ManagedAddress<M>,   
    pub arrayStakedNFTs: Vec<StakedNFT<M>>,
    pub payout: BigUint<M>,                     //accumulated payout amount
    pub last_withdraw_datetime: u64             //last datetime payout was withdraw
}



impl<M: ManagedTypeApi> StakedAddressNFTs<M>
where
    M: ManagedTypeApi,
{
    pub fn new(address: ManagedAddress<M>,
               arrayStakedNFTs: Vec<StakedNFT<M>>,
               payout: BigUint<M>,
               last_withdraw_datetime: u64
    ) -> Self {
        StakedAddressNFTs {
            address,
            arrayStakedNFTs,
            payout,
            last_withdraw_datetime
        }
    }
}


#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct StakedNFT<M>
where
    M: ManagedTypeApi,
{
    pub token_id: TokenIdentifier<M>,
    pub nonce: u64,
    pub staked_datetime: u64                //datetime it was added to staking
}

impl<M: ManagedTypeApi> StakedNFT<M>
where
    M: ManagedTypeApi,
{
    pub fn new(token_id: TokenIdentifier<M>, nonce: u64, staked_datetime: u64) -> Self {
        StakedNFT { token_id, 
                    nonce, 
                    staked_datetime 
        }
    }
}







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
    pub fn new(token_id: TokenIdentifier<M>, nonce: u64) -> Self {
        NftId { token_id, nonce }
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
    pub fn new(owner: ManagedAddress<M>, price: BigUint<M>, timestamp: u64) -> Self {
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
    pub fn new(amount: BigUint<M>, expire: u64) -> Self {
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


    #[view(getVersion)]
    #[storage_mapper("version")]
    fn version(&self) -> SingleValueMapper<ManagedBuffer>;
    


    #[view(getNftSaleInfo)]
    #[storage_mapper("nft_sale_info")]
    fn nft_sale_info(&self, nft_id: &NftId<Self::Api>)
        -> SingleValueMapper<NftSaleInfo<Self::Api>>;



    #[view(geStakedPool)]
    #[storage_mapper("staked_pool")]
    fn staked_pool(&self) -> SingleValueMapper<Self::Api, StakedPool<Self::Api>>;

        
}
