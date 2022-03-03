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
    fn deposit(&self, #[payment_amount] amount: BigUint) {
        let caller = self.blockchain().get_caller();
        self.increase_deposit(&caller, &amount);
    }

    #[endpoint(withdraw)]
    fn withdraw(&self, #[var_args] opt_amount: OptionalValue<BigUint>) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let amount = opt_amount
            .into_option()
            .unwrap_or_else(|| self.egld_deposit(&caller).get());
        self.try_decrease_deposit(&caller, &amount)?;
        self.send_egld(&caller, &amount);
        Ok(())
    }

    fn increase_deposit(&self, address: &ManagedAddress, amount: &BigUint) -> BigUint {
        let mut deposit = self.egld_deposit(address).get();
        deposit += amount;

        self.egld_deposit(address).set(&deposit);
        self.deposit_update_event(address.clone(), deposit.clone());
        deposit
    }

    fn try_decrease_deposit(
        &self,
        address: &ManagedAddress,
        amount: &BigUint,
    ) -> SCResult<BigUint> {
        let mut deposit = self.egld_deposit(address).get();

        require!(&deposit >= amount, "insuficient funds in user deposit");
        deposit -= amount;

        self.egld_deposit(address).set(&deposit);
        self.deposit_update_event(address.clone(), deposit.clone());
        Ok(deposit)
    }

    fn try_increase_decrease_deposit(
        &self,
        address: &ManagedAddress,
        to_increase: &BigUint,
        to_decrease: &BigUint,
    ) -> SCResult<BigUint> {
        let mut deposit = self.egld_deposit(address).get();
        deposit += to_increase;

        require!(&deposit >= to_decrease, "insuficient funds in user deposit");
        deposit -= to_decrease;

        self.egld_deposit(address).set(&deposit);
        self.deposit_update_event(address.clone(), deposit.clone());
        Ok(deposit)
    }
}
