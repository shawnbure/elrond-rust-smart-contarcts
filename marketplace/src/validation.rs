elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::config::BP;
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
        require!(!self.nft_sale_info(nft_id).is_empty(), "Nft not for sale");
        Ok(())
    }

    fn require_nft_not_for_sale(&self, nft_id: &NftId) -> SCResult<()> {
        require!(self.nft_sale_info(nft_id).is_empty(), "Nft is for sale");
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
        require!(self.get_platform_cut(price) != 0, "Invalid price");

        let min_price = self.asset_min_price().get();
        require!(price >= &min_price, "Price too low");
        require!(price % &min_price == 0, "Price has to be multiple of min");

        require!(price <= &self.asset_max_price().get(), "Price too high");
        Ok(())
    }

    fn require_valid_royalties(&self, token_data: &EsdtTokenData<Self::BigUint>) -> SCResult<()> {
        let platform_fee = self.get_platform_fee_percent_or_default();
        require!(
            token_data.royalties <= self.get_royalties_max_fee_percent_or_default(),
            "Royalties too big"
        );
        require!(
            &token_data.royalties + &platform_fee.into() < BP,
            "Royalties too big"
        );
        Ok(())
    }

    fn require_uris_not_empty(&self, token_data: &EsdtTokenData<Self::BigUint>) -> SCResult<()> {
        require!(!token_data.uris.is_empty(), "Empty uris");
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
}
