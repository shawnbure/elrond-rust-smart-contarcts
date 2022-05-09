#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use core::ops::Deref;


pub mod config;
use config::{PAYOUT_TIME_BLOCK, PAYOUT_TIME_BUFFER};

pub mod events;
pub mod global_op;

pub mod storage;
pub mod utils;
pub mod validation;

use storage::{StakedPool, StakedAddressNFTs, StakedNFT, NftId};


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
        if self.staked_pool().is_empty() 
        {            
            self.staked_pool().set(StakedPool::new(Vec::new()));
        }  

        //default the last payout datetime if not set
        if self.last_payout_datetime().is_empty() 
        {            
            self.last_payout_datetime().set(0u64);
        }         
                
        //add versioning
        self.version().set(&version);
    }



    // =========================================================================================
    // Set the last_payout_datetime 
    // default it to a certain start time - this will be used on the payout for payout block

    #[payable("EGLD")]
    #[only_owner]
    #[endpoint(setLastPayoutDatetime)]   
    fn set_last_payout_datetime(&self,
                                payout_datetime: u64) -> SCResult<()> 
    {
        self.last_payout_datetime().set(payout_datetime);

        Ok(()) 
    }







    // =========================================================================================
    // Enabling a TOKEN IDENTIFIER to be STAKABLE


    #[payable("EGLD")]
    #[endpoint(addStakableTokenIdentifier)]   
    fn add_stakable_token_identifier(&self,
                                     token_id: TokenIdentifier) -> SCResult<()> 
    {
        // This is enables a "collection" with that token identifer 
        // to be able to staked it's NFT 
        if self.stakable_token_identifier(&token_id).is_empty() 
        {            
            //just set a value so it's not empty
            self.stakable_token_identifier(&token_id).set(1u16);
        }
        else
        {
            return sc_error!("Token Identifier already register as Stakable.");
        }

        Ok(()) 
    }


    #[view(isTokenIdentifierStakable)]   
    fn is_token_identifier_stakable(&self,
                                    token_id: TokenIdentifier) -> bool
    {
        return self.verify_token_identifier_is_stakable(token_id);
    }





    
    // =========================================================================================
    // STAKING / UNSTAKING NFTS
    
    
    #[view(isNFTStaked)]
    fn is_nft_staked(&self, 
                     address: ManagedAddress,
                     token_id: TokenIdentifier,
                     nonce: u64) -> bool 
    {
        return self.does_nft_exist_in_staked_addresss_nfts(address, token_id, nonce);
    }

    

    #[payable("EGLD")]
    #[endpoint(stakeAddressNFT)]   
    fn stake_address_nft(&self,
                         address: ManagedAddress,
                         token_id: TokenIdentifier,
                         nonce: u64) -> SCResult<()> 
    {
        //validation
        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;
        
        // verify that token id is stakable
        if ! self.verify_token_identifier_is_stakable(token_id.clone())
        {
            return sc_error!("Token Identifer NOT Stakable.");
        }

        
        if self.last_payout_datetime().get() == 0u64 
        {
            //fail case: last_payout_datetime never set is is 0 
            return sc_error!("Last_Payout_Datetime was NEVER setup - Must set prior to staking any NFT.");
        }


        //Step 1: Check if address is the stake pool already, if not, add it
        //------------------------------------------------------------------------

        if ! self.does_address_exist_in_staked_pool(address.clone())  //does not exist
        {
            //address not in staked pool, so add it (happens on initial adress adding an NFT)
            let mut stakedPool = self.staked_pool().get();
            
            //add it to staked pool in array of managed addresses
            stakedPool.arrayStakedAddresses.push(address.clone());
            
            //set the updated stakedPool
            self.staked_pool().set(&stakedPool);

            //since it's new address, create a stakedAddressNFT for that address
            let rewardBalanceInit = BigUint::zero();    // zero initial reward balance
            let lastWithdrawDatetimeInit = 0u64;        // set last withdraw datetime to 0  
            let payoutBlockFactorTally = 0u64;          

            //set the new StakedAddressNFTs
            self.staked_address_nfts(&address).set(StakedAddressNFTs::new(Vec::new(), 
                                                                          rewardBalanceInit, 
                                                                          lastWithdrawDatetimeInit,
                                                                          payoutBlockFactorTally));  
        }


        //Step 2: Check if NFT exist yet 
        // if it does exist, check to make sure the datetime is 0u64 (zero) and update it
        //      if it's not 0u64, then it's been staked and trying to restake (throw errow)        
        // if it doesn't, then create it in the "staked_address_nfts" array of NFTId and
        //      create it in the "staked_address_nft_info" (by address and NFTId)
        //------------------------------------------------------------------------

        //create NFTId object
        let nftId = NftId::new(token_id.clone(), nonce);

        if self.does_nft_exist_in_staked_addresss_nfts(address.clone(), token_id.clone(), nonce) 
        {
            //NFT did exist before, so it's an update case

            let mut stakedNFT = self.staked_address_nft_info(&address, &nftId).get();

            if stakedNFT.staked_datetime != 0u64
            {
                //currently staked
                return sc_error!("NFT Already Staked");
            }
            else
            {
                //update stake datetime
                stakedNFT.staked_datetime = self.blockchain().get_block_timestamp();

                //set the updated stakedNFT
                self.staked_address_nft_info(&address, &nftId).set(stakedNFT);                   
            }
        }
        else
        {
            //create new stakeNFT Object - doesn't exist, so create it                
            let weightedFactor = BigUint::from(1u64);  //default as 1x
            let stakedDateTime = self.blockchain().get_block_timestamp();
            let staked_start_rollover_checked = false;
            let rollover_balance = 0u64;
            
            let newStakedNFT = StakedNFT::new(weightedFactor, stakedDateTime, staked_start_rollover_checked, rollover_balance);               

            //set the new stakedNFT
            self.staked_address_nft_info(&address, &nftId).set(newStakedNFT);                
        }

        Ok(())
    }
   



    #[payable("EGLD")]
    #[endpoint(UnstakeAddressNFT)]   
    fn unstake_address_nft(&self,
                            address: ManagedAddress,
                            token_id: TokenIdentifier,
                            nonce: u64) -> SCResult<()> 
    {
        //validation
        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;
        
        // verify that token id is stakable
        if ! self.verify_token_identifier_is_stakable(token_id.clone())
        {
            return sc_error!("Token Identifer NOT Stakable.");
        }
        
        //check to see if it was ever staked 
        if ! self.does_nft_exist_in_staked_addresss_nfts(address.clone(), token_id.clone(), nonce)
        {
            return sc_error!("NFT was NEVER Staked before.");
        }


        //NFTId object for storage access
        let nftId = NftId::new(token_id.clone(), nonce);

        let mut stakedNFT = self.staked_address_nft_info(&address, &nftId).get();

        if stakedNFT.staked_datetime == 0u64 
        {
            return sc_error!("NFT is already UnStaked.");
        }
        else
        {
            //if unstaked, then take the time of last_payout_date and current time and add to rollover
            let lastPayoutDatetime = self.last_payout_datetime().get();

            let unstaking_accured_rollover = self.blockchain().get_block_timestamp() - lastPayoutDatetime;

            //add new rollover to existing balance
            stakedNFT.rollover_balance += unstaking_accured_rollover;

            //set the updated stakedNFT
            self.staked_address_nft_info(&address, &nftId).set(stakedNFT);    
        }


        Ok(())
    }






    // =========================================================================================
    // REWARD BALANCE FUNDS
    
    #[view(getStakingRewardBalance)]
    fn get_staking_reward_balanace(&self, 
                                   address: ManagedAddress) -> BigUint 
    {
        //get stakedAddressNFT by address to get rewards
        let stakedAddressNFT = self.staked_address_nfts(&address).get();

        return stakedAddressNFT.reward_balance;
    }



    #[payable("EGLD")]
    #[endpoint(redeemStakingRewards)]   
    fn redeem_staking_rewards(&self, 
                               address: ManagedAddress) -> SCResult<()>  
    {
        let mut stakedAddressNFT = self.staked_address_nfts(&address).get();

        if stakedAddressNFT.reward_balance == BigUint::from(0u64)
        {
            return sc_error!("Staked Reward Balance is ZERO.");
        }
        else
        {            
            //send the address the reward
            self.send_egld(&address, &stakedAddressNFT.reward_balance);            

            //update the amount 
            stakedAddressNFT.reward_balance = BigUint::zero(); 
            stakedAddressNFT.last_withdraw_datetime = self.blockchain().get_block_timestamp();
            
            //set the updated stakedAddressNFT
            self.staked_address_nfts(&address).set(&stakedAddressNFT);               
        }

        Ok(())
    }    






    // =========================================================================================
    // PAYOUT PROCESS   

    #[payable("EGLD")]
    #[endpoint(disburseRewards)]
    fn disburse_rewards( &self, 
                         reward_amount: BigUint ) -> SCResult<()>  
    {       
        // Step1: Check if the datetime is within the 24 hours payout block minimal since
        //        the last_payout_datetime 
        
        //add a time buffer just in case 
        let currentDateTime_withTimeBuffer = self.blockchain().get_block_timestamp() + PAYOUT_TIME_BUFFER;

        let mut lastPayoutDatetime = self.last_payout_datetime().get();
        let mut accuredTimeFromLastPayoutDatetime = 0u64;

        if lastPayoutDatetime == 0u64 
        {
            //fail case: last_payout_datetime never set is is 0 
            return sc_error!("Last_Payout_Datetime was NEVER setup - Must set prior to doing payout");
        }
        else if (currentDateTime_withTimeBuffer - lastPayoutDatetime ) < PAYOUT_TIME_BLOCK  //check to see at least on payout time block (with added buffer)
        {
            //fail case: current datetime (with buffer) is less that the required timeframe
            return sc_error!("Payout time period is less than required time block - Try at later time.");
        }
        else
        {        
            let timeDifference = currentDateTime_withTimeBuffer - lastPayoutDatetime;

            let numOfPayoutTimeBlockFromTimeDifference = timeDifference / PAYOUT_TIME_BLOCK;

            //update NEW last_payout_datetime 
            let newLastPayoutDateTime = lastPayoutDatetime + (numOfPayoutTimeBlockFromTimeDifference * PAYOUT_TIME_BLOCK);
            self.last_payout_datetime().set(newLastPayoutDateTime);

            //get the difference between the new and old payout datetime
            accuredTimeFromLastPayoutDatetime = newLastPayoutDateTime - lastPayoutDatetime;
        }



        let mut overallTotalPayoutQualifiedStakedNFT = 0u64;

        //get the staked pool
        let stakedPool = self.staked_pool().get();  

        //keeps an in memory of arrayStakedAddressNFTs so we don't have to set it back to block and read again 
        //later on when we divide the rewards based on Qualified NFTs
        let arrayStakedAddressNFTs: Vec<StakedAddressNFTs<Self::Api>> = Vec::new();

        // iterate over the array of stakedAddresses to get address to get the 
        // stakedAddressNFTs to tally up the payout_block_factor_tally (used to split the rewards)
        for stakedAddress in stakedPool.arrayStakedAddresses 
        {  
            let mut stakedAddressNFT = self.staked_address_nfts(&stakedAddress).get();
            
            //add it to memory
            //arrayStakedAddressNFTs.push(stakedAddressNFT);

            for nftId in stakedAddressNFT.arrayStakedNFTIds
            {
                let mut stakedNFT = self.staked_address_nft_info(&stakedAddress, &nftId).get();
                
                if stakedNFT.staked_datetime != 0u64  //STILL STAKED
                {
                    //get the difference between the 
                    let accuredTimeFromLastPayoutDatetime = accuredTimeFromLastPayoutDatetime - lastPayoutDatetime;
                    
                    //add it to the rollover balance
                    stakedNFT.rollover_balance += accuredTimeFromLastPayoutDatetime;                    
                }

                //get the rolloverPayoutFactor by Int Division of balance with PAYOUT_TIME_BLOCK
                let nftRolloverPayoutFactor = stakedNFT.rollover_balance / PAYOUT_TIME_BLOCK;

                if nftRolloverPayoutFactor > 1  //factor must be greater than 1 to account for payout and updates
                {
                    //deduct the rollover balance (since it's been payout for rewards)
                    stakedNFT.rollover_balance -= (nftRolloverPayoutFactor * PAYOUT_TIME_BLOCK);

                    //update the stakedNFT
                    self.staked_address_nft_info(&stakedAddress, &nftId).set(stakedNFT);

                    //add to the stakedAddressNFT payout rollover factor 
                    stakedAddressNFT.payout_block_factor_tally += nftRolloverPayoutFactor;   
                    
                    //add to overall tally
                    overallTotalPayoutQualifiedStakedNFT += nftRolloverPayoutFactor;
                }
            }
        }



        //for stakedAddress in stakedPool.arrayStakedAddresses 
        //{  
        //    let mut stakedAddressNFT = self.staked_address_nfts(&stakedAddress).get();
       // }

        //addressPayout = daily_total_reward * (numStakedNFTForAddress / overallTotalStakedNFTQualifiedForRewards)






        // a lump sum of daily reward is sent to the SC, then there is a function
        // that disburse the funds to all staked accounts with qualified NFTS 
        // (qualified NFTs are ones that have a stakedDateTime greater 24 hours).
        // NFT is weighed equally.  However, to make it scale, added a weighted
        // factor in the StakedNFT (if 1, it's counts for 1, 2, it counts as 2, 
        // and so forth). 
        // The logic will tally up all the StakedNFT count then it will update the
        // stakedAddress "payout" field according to this formula:
        // addressPayout = daily_total_reward * (numStakedNFTForAddress / overallTotalStakedNFTQualifiedForRewards)


        /*
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
            stakedAddressNFT.reward_balance += (daily_total_reward.clone() * (totalNFTQualifiedForRewardsByAddress.clone()/overallTotalStakedNFTQualifiedForRewards.clone())); //daily_total_reward * (overallTotalStakedNFTQualifiedForRewards/overallTotalStakedNFTQualifiedForRewards);
            
            //set the updated stakedAddressNFT
            self.staked_address_nfts(&stakedAddress).set(&stakedAddressNFT);              
        }
        */


        Ok(())       
    }




}
