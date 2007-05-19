elrond_wasm::imports!();
elrond_wasm::derive_imports!();

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

    fn get_platform_cut(&self, price: &Self::BigUint) -> Self::BigUint {
        let fee = self.get_platform_fee_percent_or_default();
        price * &fee.into() / BP.into()
    }

    fn get_creator_cut(
        &self,
        price: &Self::BigUint,
        token_data: &EsdtTokenData<Self::BigUint>,
    ) -> Self::BigUint {
        price * &token_data.royalties / BP.into()
    }

    fn get_token_data(
        &self,
        token_id: &TokenIdentifier,
        nonce: u64,
    ) -> EsdtTokenData<Self::BigUint> {
        let sc_address = &self.blockchain().get_sc_address();
        self.blockchain()
            .get_esdt_token_data(sc_address, token_id, nonce)
    }

    fn safe_send(
        &self,
        to: &Address,
        token_id: &TokenIdentifier,
        nonce: u64,
        amount: &Self::BigUint,
    ) {
        if amount > &0 && to != &Address::zero() {
            self.send().direct(to, token_id, nonce, amount, &[]);
        }
    }
}