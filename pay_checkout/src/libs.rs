#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod marketplace_proxy {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait MarketPlace {
        #[endpoint(tryDecreaseDeposit)]
        fn try_decrease_deposit(
            &self,
            address: &ManagedAddress,
            amount: &BigUint,
        ) -> SCResult<BigUint>;
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
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        self.marketplace_proxy(marketplace)
            .try_decrease_deposit(caller, amount)
            .async_call()
            .call_and_exit();
    }

    #[callback]
    fn pay_checkout_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<BigUint>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(value) => {
               self.deposit().update(|deposit| *deposit += value);
            },
            ManagedAsyncCallResult::Err(_) => {
                todo!();
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

    #[proxy]
    fn marketplace_proxy(&self, sc_address: ManagedAddress) -> marketplace_proxy::Proxy<Self::Api>;
}