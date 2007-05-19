elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::config;
use super::storage;
use super::storage::NftId;
use super::storage::NftSaleInfo;
use super::utils;
use super::validation;

#[elrond_wasm::module]
pub trait ViewsModule:
    storage::StorageModule + validation::ValidationModule + config::ConfigModule + utils::UtilsModule
{
    #[view(getNftPrice)]
    fn get_nft_price(&self, token_id: TokenIdentifier, nonce: u64) -> SCResult<Self::BigUint> {
        let nft_id = NftId::new(token_id.clone(), nonce);
        self.require_nft_for_sale(&nft_id)?;
        let sale_info = self.nft_sale_info(&nft_id).get();
        Ok(sale_info.price)
    }

    #[view(getNftSaleInfo)]
    fn get_nft_sale_info(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
    ) -> SCResult<NftSaleInfo<Self::BigUint>> {
        let nft_id = NftId::new(token_id.clone(), nonce);
        self.require_nft_for_sale(&nft_id)?;
        let sale_info = self.nft_sale_info(&nft_id).get();
        Ok(sale_info)
    }
}
