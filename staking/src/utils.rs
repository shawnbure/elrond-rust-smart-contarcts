elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::storage::{AuctionInfo, NftId};

use super::config::{BP};
use super::storage;

#[elrond_wasm::module]
pub trait UtilsModule: storage::StorageModule {
 


    fn send_nft(&self, to: &ManagedAddress, token_id: &TokenIdentifier, nonce: u64) {
        self.send().direct(to, token_id, nonce, &1u64.into(), &[]);
    }

    fn send_egld(&self, to: &ManagedAddress, amount: &BigUint) {
        self.send().direct_egld(to, amount, &[]);
    }


}
