elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::config::BP;
use crate::storage::AuctionInfo;
use crate::storage::NftSaleInfo;

use super::config;
use super::storage;
use super::storage::{NftId, Offer};
use super::utils;

#[elrond_wasm::module]
pub trait ValidationModule:
    storage::StorageModule + config::ConfigModule + utils::UtilsModule
{

    fn require_valid_token_id(&self, token_id: &TokenIdentifier<Self::Api>) -> SCResult<()> {
        require!(token_id.is_valid_esdt_identifier(), "Invalid token Id");
        Ok(())
    }

    fn require_valid_nonce(&self, nonce: u64) -> SCResult<()> {
        require!(nonce != 0, "Invalid nonce");
        Ok(())
    }


}
