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

use storage::{StakedPool, StakedAddressNFTs, StakedNFT, NftId, StakedAddressPayoutTallyTracker};


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
        //DISCLAIMER: should be set once initially - if reset again, it 
        //will mess with the disbursement logic
        self.last_payout_datetime().set(payout_datetime);

        Ok(()) 
    }


    //TESTED: 5/10
    #[payable("EGLD")]
    #[only_owner]
    #[endpoint(setLastPayoutDatetimeToBlockTimestamp)]   
    fn set_last_payout_datetime_to_block_timestamp(&self) -> SCResult<()> 
    {
        //DISCLAIMER: should be set once initially - if reset again, it 
        //will mess with the disbursement logic        
        self.last_payout_datetime().set(self.blockchain().get_block_timestamp());

        Ok(()) 
    }


    // =========================================================================================
    // Add Address Role


    #[payable("EGLD")]
    #[only_owner]
    #[endpoint(setAddressRole)]   
    fn set_address_role(&self,
                         address: ManagedAddress,
                         role: u16) -> SCResult<()> 
    {
        // Enum Address Role Values:
        // 1. Deployer (1u16)
        // 2. Owner (2u16)
        // 3. Admin (3u16) 
        
        if role != 1u16 &&      // deployer role
           role != 2u16 &&      // owner role
           role != 3u16         // admin role
        {            
            //just set a value so it's not empty
            return sc_error!("Role is not valid.");
        }
        else
        {
            self.address_role(&address).set(role);
        }

        Ok(()) 
    }






    // =========================================================================================
    // Enabling a TOKEN IDENTIFIER to be STAKEABLE

    //TESTED: 5/10
    #[payable("EGLD")]
    #[endpoint(addStakableTokenIdentifier)]   
    fn add_stakable_token_identifier(&self,
                                     token_id: TokenIdentifier) -> SCResult<()> 
    {
        let address = self.blockchain().get_caller();
   
        if ! self.address_role_exists(&address)
        {            
            return sc_error!("Caller address does not have any role priviledges.");
        }

        if ! self.address_role_can_edit_stakeable_token_id(&address)
        {
            return sc_error!("Caller address role cannot make token stakeable.");
        }


        // This is enables a "collection" with that token identifer 
        // to be able to staked it's NFT 
        if self.stakable_token_identifier(&token_id).is_empty() 
        {            
            //just set a value so it's not empty
            self.stakable_token_identifier(&token_id).set(1u16);
        }
        else
        {
            return sc_error!("Token Identifier already register as Stakeable.");
        }

        Ok(()) 
    }



    #[payable("EGLD")]
    #[endpoint(removeStakableTokenIdentifier)]   
    fn remove_stakable_token_identifier(&self,
                                         token_id: TokenIdentifier) -> SCResult<()> 
    {
        let address = self.blockchain().get_caller();

        if ! self.address_role_exists(&address)
        {            
            return sc_error!("Caller address does not have any role priviledges.");
        }

        if ! self.address_role_can_edit_stakeable_token_id(&address)
        {
            return sc_error!("Caller address role cannot make token stakeable.");
        }

        // Check to see if the token id was make stakeable before
        if ! self.stakable_token_identifier(&token_id).is_empty() 
        {            
            //clear it
            self.stakable_token_identifier(&token_id).clear();
        }
        else
        {
            return sc_error!("Token Identifier was NEVER register as Stakeable.");
        }

        Ok(()) 
    }




    
    // =========================================================================================
    // STAKING / UNSTAKING NFTS
    
    //TESTED: 5/10
    #[view(isNFTStaked)]
    fn is_nft_staked(&self, 
                     address: ManagedAddress,
                     token_id: TokenIdentifier,
                     nonce: u64) -> bool 
    {
        return self.does_nft_exist_in_staked_addresss_nfts(address, token_id, nonce);
    }

    
    //TESTED: 5/10
    #[view(getStakedNFTStakedDatetime)]
    fn get_staked_nft_staked_datetime(&self, 
                                      address: ManagedAddress,
                                      token_id: TokenIdentifier,
                                      nonce: u64) -> u64 
    {
        let nftId = NftId::new(token_id.clone(), nonce);
        
        if self.staked_nft_info(&address, &nftId).is_empty() 
        {
            return 0u64;
        }
        else
        {
            let stakedNFT = self.staked_nft_info(&address, &nftId).get();

            return stakedNFT.staked_datetime;
        }
      
    }


    //TESTED: 5/10
    #[view(getStakedNFTRolloverBalance)]
    fn get_staked_nft_rollover_balance(&self, 
                                       address: ManagedAddress,
                                       token_id: TokenIdentifier,
                                       nonce: u64) -> u64 
    {
        let nftId = NftId::new(token_id.clone(), nonce);
        
        if self.staked_nft_info(&address, &nftId).is_empty() 
        {
            return 0u64;
        }
        else
        {
            let stakedNFT = self.staked_nft_info(&address, &nftId).get();

            return stakedNFT.rollover_balance;
        }
      
    }
    
    
    //TESTED: 5/10
    #[payable("EGLD")]
    #[endpoint(stakeAddressNFT)]   
    fn stake_address_nft(&self,
                         token_id: TokenIdentifier,
                         nonce: u64) -> SCResult<()> 
    {
        let address = self.blockchain().get_caller();

        //validation
        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;
        
        // verify that token id is stakeable
        if ! self.verify_token_identifier_is_stakable(token_id.clone())
        {
            return sc_error!("Token Identifer NOT Stakeable.");
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
       
            //set the new StakedAddressNFTs
            self.staked_address_nfts(&address).set(StakedAddressNFTs::new(Vec::new(), 
                                                                          rewardBalanceInit, 
                                                                          lastWithdrawDatetimeInit));  
        }


        //Step 2: Check if NFT exist yet 
        // if it does exist, check to make sure the datetime is 0u64 (zero) and update it
        //      if it's not 0u64, then it's been staked and trying to restake (throw errow)        
        // if it doesn't, then create it in the "staked_address_nfts" array of NFTId and
        //      create it in the "staked_nft_info" (by address and NFTId)
        //------------------------------------------------------------------------

        //create NFTId object
        let nftId = NftId::new(token_id.clone(), nonce);

        if self.does_nft_exist_in_staked_addresss_nfts(address.clone(), token_id.clone(), nonce) 
        {
            //NFT did exist before, so it's an update case

            let mut stakedNFT = self.staked_nft_info(&address, &nftId).get();

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
                self.staked_nft_info(&address, &nftId).set(stakedNFT);                   
            }
        }
        else
        {
            //create new stakeNFT Object - doesn't exist, so create it                
            let weightedFactor = BigUint::from(1u64);  //default as 1x
            let stakedDateTime = self.blockchain().get_block_timestamp();
            let rollover_balance = 0u64;
            
            let newStakedNFT = StakedNFT::new(weightedFactor, stakedDateTime, rollover_balance);               

            //set the new stakedNFT
            self.staked_nft_info(&address, &nftId).set(newStakedNFT);                
        }

        Ok(())
    }
   


    //TESTED: 5/10
    #[payable("EGLD")]
    #[endpoint(unstakeAddressNFT)]   
    fn unstake_address_nft(&self,
                            token_id: TokenIdentifier,
                            nonce: u64) -> SCResult<()> 
    {
        let address = self.blockchain().get_caller();

        //validation
        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;
        
        // verify that token id is stakeable
        if ! self.verify_token_identifier_is_stakable(token_id.clone())
        {
            return sc_error!("Token Identifer NOT Stakeable - So it is UNSTAKEABLE.");
        }
        
        //check to see if it was ever staked 
        if ! self.does_nft_exist_in_staked_addresss_nfts(address.clone(), token_id.clone(), nonce)
        {
            return sc_error!("NFT was NEVER Staked before.");
        }

        //NFTId object for storage access
        let nftId = NftId::new(token_id.clone(), nonce);

        let mut stakedNFT = self.staked_nft_info(&address, &nftId).get();

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
            stakedNFT.staked_datetime = 0u64;

            //set the updated stakedNFT
            self.staked_nft_info(&address, &nftId).set(stakedNFT);    
        }


        Ok(())
    }






    // =========================================================================================
    // REWARD BALANCE FUNDS
    
    //TESTED: 5/10
    #[view(getStakingRewardBalance)]
    fn get_staking_reward_balanace(&self) -> BigUint 
    {
        let address = self.blockchain().get_caller();
        
        if self.staked_address_nfts(&address).is_empty()
        {
            return BigUint::from(0u64);
        }
        else
        {
            //get stakedAddressNFT by address to get rewards
            let stakedAddressNFT = self.staked_address_nfts(&address).get();

            return stakedAddressNFT.reward_balance;
        }
    }


    //TESTED: 5/10
    #[payable("EGLD")]
    #[endpoint(redeemStakingRewards)]   
    fn redeem_staking_rewards(&self) -> SCResult<()>  
    {
        let address = self.blockchain().get_caller();

        if self.staked_address_nfts(&address).is_empty()
        {
            return sc_error!("Address has NOT staked any NFTs.");
        }

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


    #[view(qualifiedStakedNFTsForRewards)]
    fn qualified_staked_nfts_for_rewards( &self) -> u16  
    {


        //check to see if there is any nfts (currently staked or rollover time qualified for reward)

        //return 0 - No qualified NFTs found
        //return 1 - true 
        //return 2 - lastPayoutDateTime never setup
        //return 3 - payout time block is less than required time block

        // Step1: Check if the datetime is within the 24 hours payout block minimal since
        //        the last_payout_datetime 
        //add a time buffer just in case process job runs every day at midnight - buffer will account for that case
        let currentDateTime_withTimeBuffer = self.blockchain().get_block_timestamp() + PAYOUT_TIME_BUFFER;

        let lastPayoutDatetime = self.last_payout_datetime().get();
        let mut accuredTimeFromLastPayoutDatetime = 0u64;
        let mut newLastPayoutDateTime = 0u64;

        if lastPayoutDatetime == 0u64 
        {
            //fail case: last_payout_datetime never set and is 0 
            //Last_Payout_Datetime was NEVER setup - Must set prior to doing payout"
            return 2u16;
        }
        else if (currentDateTime_withTimeBuffer - lastPayoutDatetime ) < PAYOUT_TIME_BLOCK  //check to see at least on payout time block (with added buffer)
        {
            //fail case: current datetime (with buffer) is less that the required timeframe
            //Payout time period is less than required time block - Try at later time.
            return 2u16;
        }
        else
        {        
            let timeDifference = currentDateTime_withTimeBuffer - lastPayoutDatetime;

            let numOfPayoutTimeBlockFromTimeDifference = timeDifference / PAYOUT_TIME_BLOCK;

            // NEW last_payout_datetime 
            newLastPayoutDateTime = lastPayoutDatetime + (numOfPayoutTimeBlockFromTimeDifference * PAYOUT_TIME_BLOCK);

            //get the difference between the new and old payout datetime
            accuredTimeFromLastPayoutDatetime = newLastPayoutDateTime - lastPayoutDatetime;
        }

        let mut qualifiedNFTFound = 0u16;

        //get the staked pool
        let stakedPool = self.staked_pool().get();  
        
        // iterate over the array of stakedAddresses to get address to get the 
        // stakedAddressNFTs to tally up the payout_block_factor_tally (used to split the rewards)
        for stakedAddress in stakedPool.arrayStakedAddresses 
        {
            let stakedAddressNFT = self.staked_address_nfts(&stakedAddress).get();

            for nftId in stakedAddressNFT.arrayStakedNFTIds
            {
                let mut stakedNFT = self.staked_nft_info(&stakedAddress, &nftId).get();
                
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
                    qualifiedNFTFound = 1u16;
                    break;
                }                
            }

            if qualifiedNFTFound == 1u16
            {
                break;
            }
        }
                
        return qualifiedNFTFound;
    }  
    
    


    #[payable("EGLD")]
    #[endpoint(disburseRewards)]
    fn disburse_rewards( &self, 
                         reward_amount: u64 ) -> SCResult<()>  
    {       
        let address = self.blockchain().get_caller();

        if ! self.address_role_exists(&address)
        {            
            return sc_error!("Caller address does not have any role priviledges.");
        }

        if ! self.address_role_can_disburse_rewards(&address)
        {
            return sc_error!("Caller address role cannot disburse rewards.");
        }
        
        
        // Step1: Check if the datetime is within the 24 hours payout block minimal since
        //        the last_payout_datetime 
        
        //add a time buffer just in case process job runs every day at midnight - buffer will account for that case
        let currentDateTime_withTimeBuffer = self.blockchain().get_block_timestamp() + PAYOUT_TIME_BUFFER;

        let lastPayoutDatetime = self.last_payout_datetime().get();
        let mut accuredTimeFromLastPayoutDatetime = 0u64;

        //VALIDATIONS
        if lastPayoutDatetime == 0u64 
        {
            //fail case: last_payout_datetime never set and is 0 
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
            self.last_payout_datetime().set(newLastPayoutDateTime);  //verified

            //get the difference between the new and old payout datetime
            accuredTimeFromLastPayoutDatetime = newLastPayoutDateTime - lastPayoutDatetime;
        }


        let reward_amount_f64: f64 = reward_amount as f64;

        //running total of all qualified NFT for all addresses
        let mut overallTotalPayoutQualifiedStakedNFT : u64 = 0u64;

        //get the staked pool
        let stakedPool = self.staked_pool().get();  

        //keeps an in memory of address / payoutTally in StakedAddressPayoutTallyTracker and so we don't have to set 
        //it back to block and read again later on when we divide the rewards based on Qualified NFTs
        let mut arrayStakedAddressPayoutTallyTracker: Vec<StakedAddressPayoutTallyTracker<Self::Api>> = Vec::new();

        // iterate over the array of stakedAddresses to get address to get the 
        // stakedAddressNFTs to tally up the payout_block_factor_tally (used to split the rewards)
        for stakedAddress in stakedPool.arrayStakedAddresses 
        {  
            let mut stakedAddressNFT = self.staked_address_nfts(&stakedAddress).get();
            

            //store in-memory address and payout block factor tally
            let mut stakedAddressPayoutTallyTracker = StakedAddressPayoutTallyTracker::new(stakedAddress.clone(), 0u64);
           
            for nftId in stakedAddressNFT.arrayStakedNFTIds
            {
                let mut stakedNFT = self.staked_nft_info(&stakedAddress, &nftId).get();
                
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
                    self.staked_nft_info(&stakedAddress, &nftId).set(stakedNFT);

                    //add to the stakedAddressNFT payout rollover factor 
                    stakedAddressPayoutTallyTracker.payout_block_factor_tally += nftRolloverPayoutFactor;
                    
                    //add to overall tally
                    overallTotalPayoutQualifiedStakedNFT += nftRolloverPayoutFactor;
                }
            }

            //add to in-memory array
            arrayStakedAddressPayoutTallyTracker.push(stakedAddressPayoutTallyTracker);
        }



        //iterate over the in-memory vectory of address and payout tally 
        for stakedAddressPayoutTallyTracker in arrayStakedAddressPayoutTallyTracker
        {
            //only reward if there is a block factor
            if stakedAddressPayoutTallyTracker.payout_block_factor_tally > 0u64
            {
                let addressRewardPayoutAmount_f64 = (reward_amount_f64) * ( (stakedAddressPayoutTallyTracker.payout_block_factor_tally as f64) / (overallTotalPayoutQualifiedStakedNFT as f64) );

                let addressRewardPayoutAmount = addressRewardPayoutAmount_f64 as u64;

                if addressRewardPayoutAmount > 0u64  
                {
                    //only update it to the BlockChain if there is award amount
                    let mut stakedAddressNFT = self.staked_address_nfts(&stakedAddressPayoutTallyTracker.address).get();
                    stakedAddressNFT.reward_balance += addressRewardPayoutAmount;
        
                    self.staked_address_nfts(&stakedAddressPayoutTallyTracker.address).set(stakedAddressNFT);
                }
            }
        }


        Ok(())       
    }






    // =========================================================================================
    // TEST FUNCTIONS

    #[endpoint]   
    fn test_set_address_reward(&self,
                                address: ManagedAddress, 
                                reward_amount: BigUint) -> SCResult<()> 
    {
        let mut stakedAddressNFT = self.staked_address_nfts(&address).get();

        stakedAddressNFT.reward_balance += reward_amount;

        self.staked_address_nfts(&address).set(stakedAddressNFT);

        Ok(()) 
    }


    #[endpoint]   
    fn test_reset_lastpayoutdatetime(&self) -> SCResult<()> 
    {
        self.last_payout_datetime().set(0u64);

        Ok(()) 
    }


    #[view]   
    fn test_get_currenttime(&self) -> u64
    {
        return self.blockchain().get_block_timestamp();
    }
    
    
    #[view]   
    fn test_difference_lastpayoutdatetime_vs_currenttime(&self) -> u64
    {
        let currentDateTime = self.blockchain().get_block_timestamp();

        let lastPayoutDatetime = self.last_payout_datetime().get();

        return currentDateTime-lastPayoutDatetime;
    }


    #[view]   
    fn test_time_block_calculations(&self) -> u64
    {
        /*
        let currentDateTime_withTimeBuffer = self.blockchain().get_block_timestamp() + PAYOUT_TIME_BUFFER;
        let lastPayoutDatetime = self.last_payout_datetime().get();
        let timeDifference = currentDateTime_withTimeBuffer - lastPayoutDatetime;
        */

        let timeDifference = self.test_difference_lastpayoutdatetime_vs_currenttime();

        let numOfPayoutTimeBlockFromTimeDifference = timeDifference / PAYOUT_TIME_BLOCK;    

        return numOfPayoutTimeBlockFromTimeDifference;
    }
    
    
    #[view]   
    fn test_get_new_lastpayoutdatetime(&self) -> u64
    {
        /*
        let currentDateTime_withTimeBuffer = self.blockchain().get_block_timestamp() + PAYOUT_TIME_BUFFER;
        let lastPayoutDatetime = self.last_payout_datetime().get();
        let timeDifference = currentDateTime_withTimeBuffer - lastPayoutDatetime;
        */

        let numOfPayoutTimeBlockFromTimeDifference = self.test_time_block_calculations();

        let lastPayoutDatetime = self.last_payout_datetime().get();

        let newLastPayoutDateTime = lastPayoutDatetime + (numOfPayoutTimeBlockFromTimeDifference * PAYOUT_TIME_BLOCK);

        return newLastPayoutDateTime;          
    }


    /*
    #[view]
    fn test_reward_split(&self) -> BigUint
    {
        let rewardAmount: u64 = 80u64;
        let reward_amount_f64 = rewardAmount as f64;

        let overallTotalPayoutQualifiedStakedNFT: u64 = 3u64;
        let payoutBlockFactorTally: u64 = 1u64;

        let splitReturn_f64 : f64 = (reward_amount_f64) * ((payoutBlockFactorTally as f64)/(overallTotalPayoutQualifiedStakedNFT as f64));
        let splitReturn = splitReturn_f64 as u64;

        return BigUint::from(splitReturn);    
    }


    #[view]
    fn test_marketplace_fee_splits(&self) -> BigUint
    {
        let marketPlaceFees = BigUint::from(70u64);
        let marketPlaceFeesConvertU64 = marketPlaceFees.to_u64().unwrap();

        let daoPercentage: f64 = 0.20;
        let stakePercentage: f64 = 0.80;

        return BigUint::from(((marketPlaceFeesConvertU64 as f64) * stakePercentage) as u64);    
    }
    */

}
