elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::config::BP;
use crate::config::MAX_COLLECTION_NAME_LEN;
use crate::storage::NftSaleInfo;

use super::config;
use super::storage;
use super::storage::NftId;
use super::utils;

#[elrond_wasm::module]
pub trait ValidationModule:
    storage::StorageModule + config::ConfigModule + utils::UtilsModule
{
    fn require_nft_for_sale(&self, nft_id: &NftId) -> SCResult<()> {
        require!(!self.nft_sale_info(&nft_id).is_empty(), "Nft not for sale");
        Ok(())
    }

    fn require_nft_not_for_sale(&self, nft_id: &NftId) -> SCResult<()> {
        require!(self.nft_sale_info(&nft_id).is_empty(), "Nft is for sale");
        Ok(())
    }

    fn require_valid_token_id(&self, token_id: &TokenIdentifier) -> SCResult<()> {
        require!(token_id.is_valid_esdt_identifier(), "Invalid token Id");
        Ok(())
    }

    fn require_valid_nonce(&self, nonce: u64) -> SCResult<()> {
        require!(nonce != 0, "Invalid nonce");
        Ok(())
    }

    fn require_valid_amount(&self, amount: &Self::BigUint) -> SCResult<()> {
        require!(amount == &1, "Invalid amount");
        Ok(())
    }

    fn require_valid_price(&self, price: &Self::BigUint) -> SCResult<()> {
        require!(self.get_platform_cut(&price) != 0, "Invalid price");
        Ok(())
    }

    fn require_valid_royalties(&self, token_data: &EsdtTokenData<Self::BigUint>) -> SCResult<()> {
        let platform_fee = self.get_platform_fee_percent_or_default();
        require!(
            &token_data.royalties + &platform_fee.into() < Self::BigUint::from(BP),
            "Royalties too big"
        );
        Ok(())
    }

    fn require_uris_not_empty(&self, token_data: &EsdtTokenData<Self::BigUint>) -> SCResult<()> {
        require!(!token_data.uris.is_empty(), "Empty uris");
        Ok(())
    }

    fn require_good_register_collection_payment(&self, payment: &Self::BigUint) -> SCResult<()> {
        let required = self.collection_register_price().get();
        require!(payment >= &required, "Not enough payment");
        Ok(())
    }

    fn require_token_id_not_registered_already(&self, token_id: &TokenIdentifier) -> SCResult<()> {
        require!(
            self.collection_name(token_id).is_empty(),
            "Token id already registered"
        );
        Ok(())
    }

    fn require_collection_name_unique(&self, collection_name: &BoxedBytes) -> SCResult<()> {
        require!(
            !self.all_collection_names().contains(collection_name),
            "Name already taken"
        );
        Ok(())
    }

    fn require_valid_collection_name(&self, collection_name: &BoxedBytes) -> SCResult<()> {
        require!(
            !collection_name.is_empty(),
            "Collection name cannot be empty"
        );
        require!(
            collection_name.len() < MAX_COLLECTION_NAME_LEN,
            "Collection name too long"
        );
        Ok(())
    }

    fn require_owns_nft(
        &self,
        address: &Address,
        nft_sale_info: &NftSaleInfo<Self::BigUint>,
    ) -> SCResult<()> {
        require!(address == &nft_sale_info.owner, "Not owner");
        Ok(())
    }

    fn require_not_owns_nft(
        &self,
        address: &Address,
        nft_sale_info: &NftSaleInfo<Self::BigUint>,
    ) -> SCResult<()> {
        require!(address != &nft_sale_info.owner, "Is owner");
        Ok(())
    }

    fn require_good_payment(
        &self,
        payment: &Self::BigUint,
        nft_sale_info: &NftSaleInfo<Self::BigUint>,
    ) -> SCResult<()> {
        require!(payment >= &nft_sale_info.price, "Not enough payment");
        Ok(())
    }
}
