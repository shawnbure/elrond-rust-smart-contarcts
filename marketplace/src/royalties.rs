elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::storage;
use super::utils;

#[elrond_wasm::module]
pub trait RoyaltiesModule: storage::StorageModule + utils::UtilsModule {
    #[only_owner]
    #[endpoint(withdrawPlatformRoyalties)]
    fn withdraw_platform_royalties(&self, #[var_args] amount_opt: OptionalValue<Self::BigUint>) {
        let amount = amount_opt
            .into_option()elrond_wasm::imports!();
            elrond_wasm::derive_imports!();
            
            use super::storage;
            use super::utils;
            
            #[elrond_wasm::module]
            pub trait RoyaltiesModule: storage::StorageModule + utils::UtilsModule {
                #[only_owner]
                #[endpoint(withdrawPlatformRoyalties)]
                fn withdraw_platform_royalties(&self, #[var_args] amount_opt: OptionalValue<BigUint>) {
                    let amount = amount_opt
                        .into_option()
                        .unwrap_or(self.platform_royalties().get());
            
                    let caller = &self.blockchain().get_caller();
                    self.send_egld(caller, &amount);
                    self.platform_royalties().update(|x| *x -= amount);
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
                fn get_remaining_epochs_until_claim(&self, caller: ManagedAddress) -> SCResult<u64> {
                    let curr_epoch = self.blockchain().get_block_epoch();
                    let last_epoch = self.creator_last_withdrawal_epoch(&caller).get();
                    let withdrawal_epochs = self.creator_withdrawal_waiting_epochs().get();
                    require!(curr_epoch >= last_epoch, "last epoch greater than current");
            
                    let remaining = if last_epoch != 0 {
                        (withdrawal_epochs - (curr_epoch - last_epoch) % withdrawal_epochs) % withdrawal_epochs
                    } else {
                        0
                    };
            
                    Ok(remaining)
                }
            
                fn increase_platform_royalties(&self, amount: &BigUint) {
                    self.platform_royalties().update(|x| *x += amount);
                }
            
                fn increase_creator_royalties(&self, creator: &ManagedAddress, amount: &BigUint) {
                    self.creator_royalties(creator).update(|x| *x += amount);
                }
            
                fn creator_not_blacklisted(&self, address: &ManagedAddress) -> bool {
                    !self.creator_blacklist(address).get()
                }
            
                fn set_creator_last_withdrawal_epoch_if_empty(&self, creator: &ManagedAddress) {
                    if self.creator_last_withdrawal_epoch(creator).is_empty() {
                        let current = self.blockchain().get_block_epoch();
                        self.creator_last_withdrawal_epoch(creator).set(&current);
                    }
                }
            }
            
            .unwrap_or(self.platform_royalties().get());

        let caller = &self.blockchain().get_caller();
        self.send_egld(caller, &amount);
        self.platform_royalties().update(|x| *x -= amount);
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
        let curr_epoch = self.blockchain().get_block_epoch();
        let last_epoch = self.creator_last_withdrawal_epoch(&caller).get();
        let withdrawal_epochs = self.creator_withdrawal_waiting_epochs().get();
        require!(curr_epoch >= last_epoch, "last epoch greater than current");

        let remaining = if last_epoch != 0 {
            (withdrawal_epochs - (curr_epoch - last_epoch) % withdrawal_epochs) % withdrawal_epochs
        } else {
            0
        };

        Ok(remaining)
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
