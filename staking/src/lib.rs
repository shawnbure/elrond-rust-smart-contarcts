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

use storage::{StakedPool, StakedAddressNFTs, StakedNFT};
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
                
        //add versioning
        self.version().set(&version);
    }





    #[payable("EGLD")]
    #[endpoint(disburseDailyTotalReward)]
    fn disburse_daily_total_reward( &self, 
                                    daily_total_reward: BigUint ) -> SCResult<()>  
    {       
        // a lump sum of daily reward is sent to the SC, then there is a function
        // that disburse the funds to all staked accounts with qualified NFTS 
        // (qualified NFTs are ones that have a stakedDateTime greater 24 hours).
        // NFT is weighed equally.  However, to make it scale, added a weighted
        // factor in the StakedNFT (if 1, it's counts for 1, 2, it counts as 2, 
        // and so forth). 
        // The logic will tally up all the StakedNFT count then it will update the
        // stakedAddress "payout" field according to this formula:
        // addressPayout = daily_total_reward * (numStakedNFTForAddress / overallTotalStakedNFTQualifiedForRewards)


        //all staked nfts across all address
        let overallTotalStakedNFTQualifiedForRewards = self.get_overall_total_staked_nfts_qualified_for_rewards();


        //get the staked pool
        let stakedPool = self.staked_pool().get();
     
        //get the array of stakedAddresses
        let arrayStakedAddresses = stakedPool.arrayStakedAddresses;
        
        //iterate over the array of stakedAddresses to get address to get the stakedAddressNFTs to update payout amount
        for stakedAddress in arrayStakedAddresses 
        {                  
            let totalNFTQualifiedForRewardsByAddress = self.get_total_staked_nfts_qualified_for_rewards_by_address(stakedAddress.clone()); 


            let mut stakedAddressNFT = self.staked_address_nfts(&stakedAddress).get();

            //addressPayout = daily_total_reward * (numStakedNFTForAddress / overallTotalStakedNFTQualifiedForRewards)
            stakedAddressNFT.payout += (daily_total_reward.clone() * (totalNFTQualifiedForRewardsByAddress.clone()/overallTotalStakedNFTQualifiedForRewards.clone())); //daily_total_reward * (overallTotalStakedNFTQualifiedForRewards/overallTotalStakedNFTQualifiedForRewards);
            
            //set the updated stakedAddressNFT
            self.staked_address_nfts(&stakedAddress).set(&stakedAddressNFT);              
        }


        Ok(())       
    }



    #[view(getStakingRewards)]
    fn get_staking_rewards(&self, 
                           address: ManagedAddress) -> BigUint 
    {
        //get stakedAddressNFT by address to get payout
        let stakedAddressNFT = self.staked_address_nfts(&address).get();

        return stakedAddressNFT.payout;
    }




    #[payable("EGLD")]
    #[endpoint(redeemStakingRewards)]   
    fn redeem_staking_rewards(&self, 
        address: ManagedAddress) -> SCResult<()>  
    {
        let stakingRewards = self.get_staking_rewards(address.clone());

        if stakingRewards > BigUint::zero()
        {
            let mut stakedAddressNFT = self.staked_address_nfts(&address).get();

            //send the address teh payout
            self.send_egld(&address, &stakedAddressNFT.payout);

            //update the amount 
            stakedAddressNFT.payout = BigUint::zero(); 
            stakedAddressNFT.last_withdraw_datetime = self.blockchain().get_block_timestamp();

            //set the updated stakedAddressNFT
            self.staked_address_nfts(&address).set(&stakedAddressNFT);               
        }
        else
        {
            return sc_error!("Stake Rewards Balance is zero.");
        }        

        Ok(())
    }    



    #[view(isNFTStaked)]
    fn is_nft_staked(&self, 
                     address: ManagedAddress,
                     token_id: TokenIdentifier,
                     nonce: u64) -> bool 
    {
        //get stakedAddressNFT by address to get payout
        let indexOfNFTInStakedAddress = self.get_index_of_nft_in_staked_address(address.clone(), token_id.clone(), nonce);

        return indexOfNFTInStakedAddress >= 0;
    }




    #[payable("EGLD")]
    #[endpoint(removeStakedAddressNFT)]   
    fn remove_staked_address_nft(&self,
                                 address: ManagedAddress,
                                 token_id: TokenIdentifier,
                                 nonce: u64) -> SCResult<()> 
    {
        //Step 1: Check if address is the stake pool - if it's not, then it's an error
        let doesAddressExistInStakedPool= self.does_address_exist_in_staked_pool(address.clone());

        if ! doesAddressExistInStakedPool 
        {
            return sc_error!("Address is not is staked pool, so NFT was never staked.");
        }
        else
        {
            let indexOfNFTInStakedAddress = self.get_index_of_nft_in_staked_address(address.clone(), token_id.clone(), nonce);

            if indexOfNFTInStakedAddress >= 0 
            {

                //get the stakeAddressNFT by address
                let mut stakedAddressNFT = self.staked_address_nfts(&address).get();

                let indexToRemove: usize = indexOfNFTInStakedAddress as usize;

                //remove the stakedNFT from the array
                stakedAddressNFT.arrayStakedNFTs.remove(indexOfNFTInStakedAddress as usize);
                
                //set the updated stakedAddressNFT
                self.staked_address_nfts(&address).set(&stakedAddressNFT);                
            }
            else
            {
                return sc_error!("NFT was never staked.");
            }
        }


        Ok(())
    }



    #[payable("EGLD")]
    #[endpoint(addStakedAddressNFT)]   
    fn add_staked_address_nft(&self,
                              address: ManagedAddress,
                              token_id: TokenIdentifier,
                              nonce: u64) -> SCResult<()> 
    {
        //Step 1: Check if address is the stake pool already, if not, add to it
        let doesAddressExistInStakedPool= self.does_address_exist_in_staked_pool(address.clone());

        if ! doesAddressExistInStakedPool 
        {
            //address not in staked pool, so add it (happens on initial adress adding an NFT)
            let mut stakedPool = self.staked_pool().get();

            //add it staked pool
            stakedPool.arrayStakedAddresses.push(address.clone());

            //set the updated stakedPool
            self.staked_pool().set(&stakedPool);

            //since it's new, create a stakedAddressNFT
            let payoutInit = BigUint::zero();           // zero initial payout
            let lastWithdrawDatetimeInit = 0u64;        // set last withdraw datetime to 0

            //set the new StakedAddressNFTs
            self.staked_address_nfts(&address).set(StakedAddressNFTs::new(Vec::new(), payoutInit, lastWithdrawDatetimeInit));
        }


        //Step 2: Check if NFT doesn't exist yet - if it doesn't, then create it        
        let doesNFTExistInStakedAddress = self.does_nft_exist_in_staked_addresss(address.clone(), token_id.clone(), nonce);
        
        if ! doesNFTExistInStakedAddress
        {
            //doesn't exist so create it (this is success case)

            //create new stakeNFT Object
            let weightedFactor = BigUint::from(1u64);
            let stakedDateTime = self.blockchain().get_block_timestamp();
            let newStakedNFT = StakedNFT::new(token_id.clone(), nonce, weightedFactor, stakedDateTime);

            //get the stakeAddressNFT by address
            let mut stakedAddressNFT = self.staked_address_nfts(&address).get();

            //get the new stakedNFT to array
            stakedAddressNFT.arrayStakedNFTs.push(newStakedNFT);
            
            //set the updated stakedAddressNFT
            self.staked_address_nfts(&address).set(&stakedAddressNFT);
        }
        else
        {
            return sc_error!("NFT Already Staked");
        }

        Ok(())
    }
   

}
