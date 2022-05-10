#![allow(clippy::too_many_arguments)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();






#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
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



#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct StakedAddressNFTs<M>
where
    M: ManagedTypeApi,
{ 
    pub arrayStakedNFTIds: Vec<NftId<M>>,
    pub reward_balance: BigUint<M>,             //accumulated reward amount
    pub last_withdraw_datetime: u64             //last datetime reward was withdraw
}



impl<M: ManagedTypeApi> StakedAddressNFTs<M>
where
    M: ManagedTypeApi,
{
    pub fn new(arrayStakedNFTIds: Vec<NftId<M>>,
               reward_balance: BigUint<M>,
               last_withdraw_datetime: u64
            ) -> Self {
        StakedAddressNFTs {
            arrayStakedNFTIds,
            reward_balance,
            last_withdraw_datetime
        }
    }
}


#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct StakedNFT<M>
where
    M: ManagedTypeApi,
{
    pub weighted_factor: BigUint<M>,            //if it's 1, count as 1, if 2, counts as 2x, and so forth
    pub staked_datetime: u64,                    //datetime it was added to staking
    pub rollover_balance: u64
    
}

impl<M: ManagedTypeApi> StakedNFT<M>
where
    M: ManagedTypeApi,
{
    pub fn new(weighted_factor: BigUint<M>,
               staked_datetime: u64,
               rollover_balance: u64
               ) -> Self {
        StakedNFT { weighted_factor,
                    staked_datetime,
                    rollover_balance
        }
    }
}



/*
    NFT (TokenID, Nonce)
    --------------------------------------------------
    weighted_factor: BigUint
        - count x-amount per 24 hr unit
    staked_datetime: u64
        - set when staked - if it's removed, set to zero (0).
          we have to do "soft" deletes for staking and unstalking to not lose accured staked time
    rollover_balance: 
        - holds the time accured that is paid for 


    rollover_balance RULES:
         1) ????? if ! staked_start_rollover_checked && staked_datetime < payoutDatetime, get the time prior
         2) if unstaked, then take the time of last_payout_date and current time and add to rollover



    last_payout_date
    --------------------------------------------------
    - variable that holds the last payout datetime
    - on next time payout, do a floor function to get an even units of a day (24 hours), 
        reason to do this is to make the distributions of funds to NFTS address easier
        for example: let say, last_payout_datetime is Jan 1 @ 12:00am
            if next payout call is jan 2 @ 12:00am, so set last_payout_datetime to 1/2 @ 12am
            if next payout call is jan 2 @ 11am, so set last_payout_datetime to 1/2 @12 am (floor())
            if next payout call is jan 2 @ 11pm, so set last_payout_datetime to 1/2 @12 am (floor())
            if next payout call is jan 3 @ 5am, so set last_payout_datetime to 1/3 @12 am (floor())
            if next payout call is jan 1 @ 11:59pm, throw an error and say the last_payout_datetime MUST be at least 1 day-unit.
                - reason we throw this error is to prevent multiple calls
    - base on the logic above, should set the last_payout_datetime to at least a day from next payout datetime                 
    - this ensure an even unit of "days" to split between the NFTS therefore making the disbursement much easier



*/



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
pub struct StakedAddressPayoutTallyTracker<M>
where
    M: ManagedTypeApi,
{ 
    pub address:  ManagedAddress<M>,
    pub payout_block_factor_tally: u64          //holds the payout block factors tally during the disbursement of rewards logic
}



impl<M: ManagedTypeApi> StakedAddressPayoutTallyTracker<M>
where
    M: ManagedTypeApi,
{
    pub fn new(address: ManagedAddress<M>,
               payout_block_factor_tally: u64
            ) -> Self {
                StakedAddressPayoutTallyTracker {
            address,
            payout_block_factor_tally
        }
    }
}




#[elrond_wasm_derive::module]
//#[elrond_wasm::module]
pub trait StorageModule {

    #[view(geStakedPool)]
    #[storage_mapper("staked_pool")]
    fn staked_pool(&self) -> SingleValueMapper<StakedPool<Self::Api>>;


    #[view(geStakedAddressNFTs)]
    #[storage_mapper("staked_address_nfts")]
    fn staked_address_nfts(&self, address: &ManagedAddress) -> SingleValueMapper<StakedAddressNFTs<Self::Api>>;


    #[view(geStakedNFTInfo)]
    #[storage_mapper("staked_nft_info")]
    fn staked_nft_info(&self, address: &ManagedAddress, nft_id: &NftId<Self::Api>) -> SingleValueMapper<StakedNFT<Self::Api>>;





    #[view(getLastPayoutDatetime)]
    #[storage_mapper("last_payout_datetime")]
    fn last_payout_datetime(&self) -> SingleValueMapper<Self::Api, u64>;




    // set to a value of 1 if tokenIdentifier is stakeable
    // this verify that a token is allowed to be staked
    #[view(geStakedTokenIndentifier)]
    #[storage_mapper("stakable_token_identifier")]
    fn stakable_token_identifier(&self, token_identifier: &TokenIdentifier) -> SingleValueMapper<Self::Api, u16>;


    #[view(getAdminAddress)]
    #[storage_mapper("admin_address")]
    fn admin_address(&self, address: &ManagedAddress) -> SingleValueMapper<Self::Api, u16>;


    #[view(getVersion)]
    #[storage_mapper("version")]
    fn version(&self) -> SingleValueMapper<ManagedBuffer>;

}
