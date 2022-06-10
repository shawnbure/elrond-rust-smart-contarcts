#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod status;
use status::CheckoutStatus;

use marketplace::ProxyTrait as _;

#[elrond_wasm::contract]
pub trait CheckoutDeposit {
    #[init]
    fn init(&self) {
    }

    #[endpoint(payCheckout)]
    fn pay_checkout(
        &self,
        marketplace: ManagedAddress,
        amount: BigUint,
        checkout_id: BigUint,
    ) -> SCResult<()> {
        self.status(&checkout_id).set(CheckoutStatus::Pending);
        let scResult = self.proxy_call(marketplace, amount);
        scResult
    } 

    fn proxy_call(
        &self,
        marketplace: ManagedAddress,
        amount: BigUint,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        self.marketplace_proxy(marketplace)
            .external_trusted_payment_sc(&caller, &amount)
            .async_call()
            .call_and_exit();
    }

    #[callback]
    fn pay_checkout_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<BigUint>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(checkout_id) => {
            //    self.deposit().update(|deposit| *deposit += value);
               self.status(&checkout_id).set(CheckoutStatus::Successful);
            },
            ManagedAsyncCallResult::Err(_) => {
                // self.status(&checkout_id).set(CheckoutStatus::Failed);
                // self.err_storage().set(&err.err_msg);
            },
        }
    }

    #[endpoint(withdraw)]
    fn withdraw(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        require!(caller == self.blockchain().get_owner_address(), "Only owner can withdraw!");
        let sc_balance = self.blockchain().get_sc_balance(&TokenIdentifier::egld(), 0);
        self.send().direct(&caller, &TokenIdentifier::egld(), 0, &sc_balance, b"withdraw");
        Ok(())
    }


    // Storages
    #[view(getStatus)]
    #[storage_mapper("status")]
    fn status(&self, checkout_id: &BigUint) -> SingleValueMapper<CheckoutStatus>;

    #[proxy]
    fn marketplace_proxy(&self, sc_address: ManagedAddress) -> marketplace::Proxy<Self::Api>;
}