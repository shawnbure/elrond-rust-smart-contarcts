#![allow(clippy::too_many_arguments)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();






#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct StakedPool<M:ManagedTypeApi>
{  
    pub arrayStakedAddresses: Vec<ManagedAddress<M>>
}


impl<M: ManagedTypeApi> StakedPool<M>
where
    M: ManagedTypeApi,
{
    pub fn new(arrayStakedAddresses: Vec<ManagedAddress<M>>
    ) -> Self {
        StakedPool {
            arrayStakedAddresses
        }
    }
}



#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct StakedAddressNFTs<M>
where
    M: ManagedTypeApi,
{ 
    pub arrayStakedNFTs: Vec<StakedNFT<M>>,
    pub payout: BigUint<M>,                     //accumulated payout amount
    pub last_withdraw_datetime: u64             //last datetime payout was withdraw
}



impl<M: ManagedTypeApi> StakedAddressNFTs<M>
where
    M: ManagedTypeApi,
{
    pub fn new(arrayStakedNFTs: Vec<StakedNFT<M>>,
               payout: BigUint<M>,
               last_withdraw_datetime: u64
            ) -> Self {
        StakedAddressNFTs {
            arrayStakedNFTs,
            payout,
            last_withdraw_datetime
        }
    }
}


#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct StakedNFT<M>
where
    M: ManagedTypeApi,
{
    pub token_id: TokenIdentifier<M>,
    pub nonce: u64,
    pub weighted_factor: BigUint<M>,       //if it's 1, count as 1, if 2, counts as 2x, and so forth
    pub staked_datetime: u64            //datetime it was added to staking
    
}

impl<M: ManagedTypeApi> StakedNFT<M>
where
    M: ManagedTypeApi,
{
    pub fn new(token_id: TokenIdentifier<M>, 
               nonce: u64, 
               weighted_factor: BigUint<M>,
               staked_datetime: u64
               ) -> Self {
        StakedNFT { token_id, 
                    nonce, 
                    weighted_factor,
                    staked_datetime 
        }
    }
}






#[elrond_wasm_derive::module]
//#[elrond_wasm::module]
pub trait StorageModule {


    #[view(getVersion)]
    #[storage_mapper("version")]
    fn version(&self) -> SingleValueMapper<ManagedBuffer>;
    

    #[view(geStakedPool)]
    #[storage_mapper("staked_pool")]
    fn staked_pool(&self) -> SingleValueMapper<StakedPool<Self::Api>>;


    #[view(geStakedAddressNFTs)]
    #[storage_mapper("staked_address_nfts")]
    fn staked_address_nfts(&self, address: &ManagedAddress) -> SingleValueMapper<StakedAddressNFTs<Self::Api>>;

}
