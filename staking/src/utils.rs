elrond_wasm::imports!();
elrond_wasm::derive_imports!();


//use super::config::{BP};
use super::storage;
//use super::storage::{StakedPool, StakedAddressNFTs, StakedNFT, NftId};

#[elrond_wasm::module]
pub trait UtilsModule: storage::StorageModule {
 



 
    



    /*
    
        /*
        //get the staked pool
        let stakedPool = self.staked_pool().get();
     
        //get the array of stakedAddresses
        let arrayStakedAddresses = stakedPool.arrayStakedAddresses;

        //bool return value
        let mut isStakedAddressFound = false;

        //iterate over the array of stakedAddresses to see if address is in there already
        for stakedAddress in arrayStakedAddresses 
        {
            //check if address already exist in stakedAddress
            if stakedAddress == address  
            {
                isStakedAddressFound = true;
                break;                
            }
        }

        return isStakedAddressFound;
        */



    fn get_index_of_nft_in_staked_address_nfts(&self, 
                                                address: ManagedAddress,
                                                token_id: TokenIdentifier,
                                                nonce: u64) -> i32 
    {
        //get the staked address NFTs
        let stakedAddressNFT = self.staked_address_nfts(&address).get();

        //get the array of NFTId
        let arrayStakedNFTIds = stakedAddressNFT.arrayStakedNFTIds;

        let mut stakedNFTIndex: i32 = -1;
        let mut currentIndex: i32 = 0;

        //iterate over the array of StakedAddressNFTs to see if address is in there already
        for nftId in arrayStakedNFTIds 
        {            
            //check the token id and nonce 
            if nftId.token_id == token_id && nftId.nonce == nonce
            {
                stakedNFTIndex = currentIndex;
                break;
            }

            //increment
            currentIndex += 1;
        }

        return stakedNFTIndex;
    }





    fn get_overall_total_staked_nfts_qualified_for_rewards(&self) -> BigUint
    {
        //get addresses from the StakedPool
        //iterate over the addresses and get the StakedAddressNFTs by address 
        //  - check the stakedAddressNFTs array of NFTs if they qualifies for reward, if so add it to total

        let mut totalNFTQualifiedForRewards = BigUint::zero();

        //get the staked pool
        let stakedPool = self.staked_pool().get();
     
        //get the array of stakedAddresses
        let arrayStakedAddresses = stakedPool.arrayStakedAddresses;
        
        //iterate over the array of stakedAddresses to get address to get the stakedAddressNFTs
        for stakedAddress in arrayStakedAddresses 
        {
            totalNFTQualifiedForRewards += self.get_total_staked_nfts_qualified_for_rewards_by_address(stakedAddress.clone());                  
        }

        return totalNFTQualifiedForRewards;       
    }



    fn get_total_staked_nfts_qualified_for_rewards_by_address(&self, 
                                                               address: ManagedAddress) -> BigUint
    {
        let mut totalNFTQualifiedForRewards = BigUint::zero();

        let stakedAddressNFT = self.staked_address_nfts(&address).get();

        //get the array of stakedAddressNFTs
        let arrayStakedNFTIds = stakedAddressNFT.arrayStakedNFTIds;

        //iterate over the array of StakedAddressNFTs to see if address is in there already
        for stakedNFT in arrayStakedNFTIds 
        {
            //check if stakedNFT datetime qualifies, if so, add to the totalNFTQualiedForReward
            if self.is_stake_datetime_qualified_for_rewards(stakedNFT.staked_datetime)
            {
                //use the weighted factor
                totalNFTQualifiedForRewards += BigUint::from(stakedNFT.weighted_factor);
            }
        }   
        
        return totalNFTQualifiedForRewards;
    }                                                      



    fn is_stake_datetime_qualified_for_rewards(&self,
                                               staked_datetime: u64) -> bool   
    {
        //verify the staked datetime is greater than 24 hours
        let currentTimestamp = self.blockchain().get_block_timestamp();

        let day24Hour = 24*60*60;

        return (currentTimestamp - staked_datetime) > day24Hour;
    }

    
    */









    
    fn send_nft(&self, to: &ManagedAddress, token_id: &TokenIdentifier, nonce: u64) {
        self.send().direct(to, token_id, nonce, &1u64.into(), &[]);
    }

    fn send_egld(&self, to: &ManagedAddress, amount: &BigUint) {
        self.send().direct_egld(to, amount, &[]);

        
    }


}
