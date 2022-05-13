elrond_wasm::imports!();
elrond_wasm::derive_imports!();

//use crate::config::BP;


use super::config;
use super::storage;
use super::storage::{StakedPool, StakedAddressNFTs, StakedNFT, NftId};
use super::utils;


#[elrond_wasm::module]
pub trait ValidationModule:
    storage::StorageModule + config::ConfigModule + utils::UtilsModule
{


    fn does_address_exist_in_staked_pool(&self, 
                                         address: ManagedAddress) -> bool                                               
    {
        // since staked_address_nfts gets created if an NFT is staked with 
        // an address that isn't staked yet (look in fn stake_address_nft logic)
        return ! self.staked_address_nfts(&address).is_empty();
    }
    


    
    fn does_nft_exist_in_staked_addresss_nfts(&self, 
                                              address: ManagedAddress,
                                              token_id: TokenIdentifier,
                                              nonce: u64) -> bool                                               
    {
        // since staked_nft_info gets created if an NFT is staked with 
        // an NFTId that isn't staked (look in fn stake_address_nft logic)

        //create NFTId object
        let nftId = NftId::new(token_id.clone(), nonce);

        return ! self.staked_nft_info(&address, &nftId).is_empty();
    }

    

    
    fn verify_token_identifier_is_stakable(&self,
                                           token_id: TokenIdentifier) -> bool
    {
        return ! self.stakable_token_identifier(&token_id).is_empty();
    }





    fn address_role_exists(&self, address: &ManagedAddress) -> bool
    {
        return ! self.address_role(&address).is_empty();
    }


    fn address_role_is_deployer(&self, address: &ManagedAddress) -> bool
    {
        let addressRole = self.address_role(&address).get();

        return addressRole == 1u16;
    }
    
    fn address_role_is_owner(&self, address: &ManagedAddress) -> bool
    {
        let addressRole = self.address_role(&address).get();

        return addressRole == 2u16;
    }


    fn address_role_is_admin(&self, address: &ManagedAddress) -> bool
    {
        let addressRole = self.address_role(&address).get();

        return addressRole == 3u16;
    }


    fn address_role_can_edit_stakeable_token_id(&self, address: &ManagedAddress) -> bool
    {
        return self.address_role_is_deployer(&address) || 
               self.address_role_is_owner(&address) ||
               self.address_role_is_admin(&address);
    }

    fn address_role_can_disburse_rewards(&self, address: &ManagedAddress) -> bool
    {
        return self.address_role_is_deployer(&address) || 
               self.address_role_is_owner(&address) ||
               self.address_role_is_admin(&address);
    }

 
 

    fn require_valid_token_id(&self, token_id: &TokenIdentifier<Self::Api>) -> SCResult<()> {
        require!(token_id.is_valid_esdt_identifier(), "Invalid token Id");
        Ok(())
    }

    fn require_valid_nonce(&self, nonce: u64) -> SCResult<()> {
        require!(nonce != 0, "Invalid nonce");
        Ok(())
    }



}
