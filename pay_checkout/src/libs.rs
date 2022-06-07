#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod status;
use status::CheckoutStatus;

mod marketplace_proxy {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait MarketPlace {
        #[endpoint(checkoutPayment)]
        fn checkoutPayment(
            &self, 
            address: &ManagedAddress, 
            amount: &BigUint,
            checkout_id: &BigUint,
        ) -> SCResult<()>;
    }
}

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
        let caller = self.blockchain().get_caller();
        self.marketplace_proxy(marketplace)
            .checkoutPayment(caller, amount, checkout_id)
            .async_call()
            .with_callback(self.callbacks().pay_checkout_callback())
            .call_and_exit();
        self.status(&checkout_id).set(CheckoutStatus::Pending);
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
                self.status(&checkout_id).set(CheckoutStatus::Failed);
                // self.err_storage().set(&err.err_msg);
            },
        }
    }

    #[endpoint(withdraw)]
    fn withdraw(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        require!(caller == self.blockchain().get_owner_address(), "Only owner can withdraw!");
        let sc_balance = self.deposit().get();
        self.send().direct(&caller, &TokenIdentifier::egld(), 0, &sc_balance, b"withdraw");
        self.deposit().clear();
        Ok(())
    }


    // Storages
    #[storage_mapper("deposit")]
    fn deposit(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("status")]
    fn status(&self, checkout_id: &BigUint) -> SingleValueMapper<CheckoutStatus>;

    #[proxy]
    fn marketplace_proxy(&self, sc_address: ManagedAddress) -> marketplace_proxy::Proxy<Self::Api>;
}