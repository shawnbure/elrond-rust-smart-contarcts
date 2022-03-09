elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::storage::{AuctionInfo, NftId};

use super::config::{BP, DEFAULT_FEE_PERCENT, ROYALTIES_MAX_FEE_PERCENT};
use super::storage;

#[elrond_wasm::module]
pub trait UtilsModule: storage::StorageModule {
    fn get_platform_fee_percent_or_default(&self) -> u64 {
        let fee = self.platform_fee_percent().get();

        if fee != 0 {
            fee
        } else {
            DEFAULT_FEE_PERCENT
        }
    }

    fn get_royalties_max_fee_percent_or_default(&self) -> u64 {
        let fee = self.royalties_max_fee_percent().get();

        if fee != 0 {
            fee
        } else {
            ROYALTIES_MAX_FEE_PERCENT
        }
    }

    fn get_platform_cut(&self, price: &BigUint) -> BigUint {
        let fee = self.get_platform_fee_percent_or_default();
        price * &fee.into() / BP.into()
    }

    fn get_creator_cut(
        &self,
        price: &BigUint,
        token_data: &EsdtTokenData<BigUint>,
    ) -> BigUint {
        price * &token_data.royalties / BP.into()
    }

    fn get_token_data(
        &self,
        token_id: &TokenIdentifier,
        nonce: u64,
    ) -> EsdtTokenData<BigUint> {
        let sc_address = &self.blockchain().get_sc_address();
        self.blockchain()
            .get_esdt_token_data(sc_address, token_id, nonce)
    }

    fn send_nft(&self, to: &ManagedAddress, token_id: &TokenIdentifier, nonce: u64) {
        self.send().direct(to, token_id, nonce, &1u64.into(), &[]);
    }

    fn send_egld(&self, to: &ManagedAddress, amount: &BigUint) {
        self.send().direct_egld(to, amount, &[]);
    }

    fn auction_has_winner(&self, auction_info: &AuctionInfo<BigUint>) -> bool {
        auction_info.highest_bidder != ManagedAddress::zero()
    }

    fn is_nft_for_sale(&self, nft_id: &NftId) -> bool {
        !self.nft_sale_info(nft_id).is_empty()
    }

    fn is_nft_on_auction(&self, nft_id: &NftId) -> bool {
        !self.auction(nft_id).is_empty()
    }

    fn error_nft_not_found(&self) -> SCResult<()> {
        sc_error!("Nft not found")
    }
}
