#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use core::ops::Deref;


pub mod config;

pub mod events;
pub mod global_op;

pub mod storage;
pub mod utils;
pub mod validation;

use storage::{AuctionInfo, NftId, NftSaleInfo, Offer, StakedPool, StakedAddressNFTs, StakedNFT};
const SECONDS_IN_YEARS: u64 = 31_556_952u64;

//const LAST_WITHDRAW_DATETIME_INIT: u64 = 0;

#[elrond_wasm::contract]
pub trait StakingContract:
    events::EventsModule
    + storage::StorageModule
    + validation::ValidationModule
    + config::ConfigModule
    + utils::UtilsModule
    + global_op::GlobalOperationModule
{
    #[init]
    fn init(
        &self,
        version: ManagedBuffer,
    ) {

        //create the staked pool if it does not exist
        if self.staked_pool().is_empty() {
            
            self.staked_pool().set(StakedPool::new(Vec::new()));
        }  
                
        self.version().set(&version);
     
    }




 


    #[payable("EGLD")]
    #[endpoint(depositStaking)]
    fn deposit_staking( &self, 
                        amount: BigUint ) -> SCResult<usize>
    {       
        //caller address (since minting_limit is based on address)
        let caller_address = &self.blockchain().get_caller();

        let mut vec = Vec::new();
        vec.push(caller_address);
        vec.push(caller_address);

        Ok(vec.len())              
    }


   
    fn addAddressNFT(&self,
        address: ManagedAddress,
        token_id: TokenIdentifier,
        nonce: u64,)
    {
        //get the staked pool
        let stakedPool = self.staked_pool().get();

        //get the array of stakedAddressNFTs
        let arrayStakedAddressNFTs = stakedPool.arrayStakedAddressNFTs;


        let mut isStakedAddressNFTsFound = false;

        //iterate over the array of StakedAddressNFTs to see if address is in there already
        for stakedAddressNFTs in arrayStakedAddressNFTs 
        {
            //check if address already exist in stakedAddress
            if stakedAddressNFTs.address == address  
            {
                isStakedAddressNFTsFound = true;

                //now check to see if NFT is in arrayStakedNFTs
                
                let mut isStakedNFTFound = false;

                //iterate over the array of StakeNFTs if it's been staked already
                //let mut array2 = stakedAddressNFTs.arrayStakedNFTs;

                for stakedNFT in stakedAddressNFTs.arrayStakedNFTs
                {
                    if stakedNFT.token_id == token_id && stakedNFT.nonce == nonce
                    {
                        isStakedNFTFound = true;
                        break;
                    }
                }

                //if not staked, then add it 
                if ! isStakedNFTFound
                {
                    let stakedDateTime = self.blockchain().get_block_timestamp();

                    let newStakedNFT = StakedNFT::new(token_id.clone(), nonce, stakedDateTime);
        
                    

                    //stakedAddressNFTs.arrayStakedNFTs.push(newStakedNFT);
                }

                break;
            }
        }


        if ! isStakedAddressNFTsFound
        {
            //create new address

            let payoutInit = BigUint::zero(); 
            let stakedDateTime = self.blockchain().get_block_timestamp();

            let stakedNFT = StakedNFT::new(token_id.clone(), nonce, stakedDateTime);

            let mut arrayStakedNFTs = Vec::new();
            arrayStakedNFTs.push(stakedNFT);


            let stakedAddressNFTs = StakedAddressNFTs::new(address.clone(), arrayStakedNFTs, payoutInit, 0u64);

            //let arrayStakedAddressNFTsMod = stakedPool.arrayStakedAddressNFTs;
            //arrayStakedAddressNFTsMod.push(stakedAddressNFTs);
        }

        //iterate over the address to see if it exists
        // - if doesn't exist, then create StakedAddressNFTs, then add NFT to vector
        // - if exist, check if the NFT exist in vectorNFTIDs (for duplicate cases)
        //      - if it doesn't exist, create

        /*

        //add NFT (tokenID + nonce) to address

        let payoutInit = BigUint::zero(); 

        //let big_one = BigUint::from(1u64);
        //let big_zero = BigUint::zero();

        let timestamp = self.blockchain().get_block_timestamp();
        
        let stakedNFT1 = StakedNFT::new(token_id.clone(), nonce, timestamp);
        let stakedNFT2 = StakedNFT::new(token_id.clone(), nonce, timestamp);

        let mut vec = Vec::new();
        vec.push(stakedNFT1);
        vec.push(stakedNFT2);


       let stakedAddressNFTs_1 = StakedAddressNFTs::new(address, vec, payoutInit);
       //let stakedAddressNFTs_2 = StakedAddressNFTs::new(address, vec, payoutInit); 
        */
    }
   



}
