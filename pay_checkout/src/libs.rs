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
        fn checkout_payment(
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
    ) -> SCResult<()>{
        self.status(&checkout_id).set(CheckoutStatus::Pending);
        self.proxy_call(marketplace, amount, checkout_id)
    } 

    fn proxy_call(
        &self,
        marketplace: ManagedAddress,
        amount: BigUint,
        checkout_id: BigUint,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        self.marketplace_proxy(marketplace)
            .checkout_payment(&caller, &amount, &checkout_id)
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
        let sc_balance = self.deposit().get();
        self.send().direct(&caller, &TokenIdentifier::egld(), 0, &sc_balance, b"withdraw");
        self.deposit().clear();
        Ok(())
    }


    // Storages
    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self) -> SingleValueMapper<BigUint>;

    #[view(getStatus)]
    #[storage_mapper("status")]
    fn status(&self, checkout_id: &BigUint) -> SingleValueMapper<CheckoutStatus>;

    #[proxy]
    fn marketplace_proxy(&self, sc_address: ManagedAddress) -> marketplace_proxy::Proxy<Self::Api>;
}