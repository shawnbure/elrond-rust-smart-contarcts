elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::events;
use super::storage;
use super::utils;

#[elrond_wasm::module]
pub trait DepositModule:
    storage::StorageModule + utils::UtilsModule + events::EventsModule
{
    #[payable("EGLD")]
    #[endpoint(deposit)]
    fn deposit(&self, #[payment_amount] amount: Self::BigUint) {
        let caller = self.blockchain().get_caller();
        let amount = self.increate_deposit(&caller, &amount);
        self.deposit_update(caller, amount);
    }

    #[endpoint(withdraw)]
    fn withdraw(&self) {
        let caller = self.blockchain().get_caller();
        let amount = &self.egld_deposit(&caller).get();
        self.send_egld(&caller, amount);
        self.egld_deposit(&caller).clear();
        self.deposit_update(caller, Self::BigUint::zero());
    }

    fn increate_deposit(&self, address: &Address, amount: &Self::BigUint) -> Self::BigUint {
        let mut deposit = self.egld_deposit(address).get();
        deposit += amount;

        self.egld_deposit(address).set(&deposit);
        deposit
    }

    fn try_decrease_deposit(
        &self,
        address: &Address,
        amount: &Self::BigUint,
    ) -> SCResult<Self::BigUint> {
        let mut deposit = self.egld_deposit(address).get();

        require!(&deposit >= amount, "insuficient funds in user deposit");
        deposit -= amount;

        self.egld_deposit(address).set(&deposit);
        Ok(deposit)
    }

    fn try_increase_decrease_deposit(
        &self,
        address: &Address,
        to_increase: &Self::BigUint,
        to_decrease: &Self::BigUint,
    ) -> SCResult<Self::BigUint> {
        let mut deposit = self.egld_deposit(address).get();
        deposit += to_increase;

        require!(&deposit >= to_decrease, "insuficient funds in user deposit");
        deposit -= to_decrease;

        self.egld_deposit(address).set(&deposit);
        Ok(deposit)
    }
}
