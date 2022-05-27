#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy)]
pub enum CheckoutStatus {
    Pending,
    Successful,
    Failed,
}

#[elrond_wasm::contract]
pub trait CheckoutDeposit {
    #[init]
    fn init(&self) {
    }

    #[endpoint]
    fn create_checkout(
        &self,
        checkout_id: BigUint,
        checkout_amount: BigUint,
    ) {
        self.checkout_info(&checkout_id).set(&checkout_amount);
        self.checkout_status(&checkout_id).set(CheckoutStatus::Pending);
    }

    #[payable("EGLD")]
    #[endpoint]
    fn pay_checkout(
        &self,
        checkout_id: BigUint,
        #[payment_amount] payment: BigUint,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        require!(self.checkout_info(&checkout_id).get() == payment, "You should pay enough!");
        require!(self.checkout_status(&checkout_id).get() == CheckoutStatus::Pending, "Is not available at the moment");
        self.checkout_status(&checkout_id).set(CheckoutStatus::Successful);
        self.user_checkouts(&caller).push(&checkout_id);
        Ok(())
    }

    #[endpoint]
    fn withdraw(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        require!(caller == self.blockchain().get_owner_address(), "Only owner can withdraw!");
        let sc_balance = self.blockchain().get_sc_balance(&TokenIdentifier::egld(), 0);
        self.send().direct(&caller, &TokenIdentifier::egld(), 0, &sc_balance, b"withdraw");

        Ok(())
    }


    // Storages
    #[view(getCheckoutStatus)]
    #[storage_mapper("checkoutStatus")]
    fn checkout_status(&self, checkout_id: &BigUint) -> SingleValueMapper<CheckoutStatus>;

    #[view(getCheckoutInfo)]
    #[storage_mapper("checkoutInfo")]
    fn checkout_info(&self, checkout_id: &BigUint) -> SingleValueMapper<BigUint>;

    #[storage_mapper("userCheckouts")]
    fn user_checkouts(&self, user: &ManagedAddress) -> VecMapper<BigUint>;
}