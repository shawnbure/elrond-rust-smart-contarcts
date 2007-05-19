elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::storage;
use super::utils;

#[elrond_wasm::module]
pub trait RoyaltiesModule: storage::StorageModule + utils::UtilsModule {
    #[only_owner]
    #[endpoint(withdrawPlatformRoyalties)]
    fn withdraw_platform_royalties(&self) {
        let caller = &self.blockchain().get_caller();
        let royalties = &self.platform_royalties().get();
        self.send_egld(caller, royalties);
        self.platform_royalties().clear();
    }

    #[endpoint(withdrawCreatorRoyalties)]
    fn withdraw_creator_royalties(&self) -> SCResult<()> {
        let caller = &self.blockchain().get_caller();
        require!(self.creator_not_blacklisted(caller), "blacklisted");

        let current_epoch = self.blockchain().get_block_epoch();
        let creator_withdrawal_waiting_epochs = self.creator_withdrawal_waiting_epochs().get();
        let last_withdrawal_epoch = self.creator_last_withdrawal_epoch(caller).get();
        require!(
            current_epoch == last_withdrawal_epoch + creator_withdrawal_waiting_epochs,
            "withdrawal called too early"
        );

        let royalties = &self.creator_royalties(caller).get();
        self.send_egld(caller, royalties);

        self.creator_royalties(caller).clear();
        self.creator_last_withdrawal_epoch(caller)
            .set(&current_epoch);
        Ok(())
    }

    #[view(getRemainingEpochsUntilClaim)]
    fn get_remaining_epochs_until_claim(&self, caller: Address) -> SCResult<u64> {
        let last_epoch = self.creator_last_withdrawal_epoch(&caller).get();
        let curr_epoch = self.blockchain().get_block_epoch();

        require!(curr_epoch >= last_epoch, "last epoch greater than current");

        let curr_last_diff = curr_epoch - last_epoch;
        let withdrawal_epochs = self.creator_withdrawal_waiting_epochs().get();
        let remaining_epochs;

        if curr_last_diff < withdrawal_epochs {
            remaining_epochs = withdrawal_epochs - curr_epoch;
        } else {
            remaining_epochs = withdrawal_epochs - (curr_last_diff - withdrawal_epochs);
        }

        Ok(remaining_epochs)
    }

    fn increase_platform_royalties(&self, amount: &Self::BigUint) {
        self.platform_royalties().update(|x| *x += amount);
    }

    fn increase_creator_royalties(&self, creator: &Address, amount: &Self::BigUint) {
        self.creator_royalties(creator).update(|x| *x += amount);
    }

    fn creator_not_blacklisted(&self, address: &Address) -> bool {
        !self.creator_blacklist(address).get()
    }

    fn set_creator_last_withdrawal_epoch_if_empty(&self, creator: &Address) {
        if self.creator_last_withdrawal_epoch(creator).is_empty() {
            let current = self.blockchain().get_block_epoch();
            self.creator_last_withdrawal_epoch(creator).set(&current);
        }
    }
}
