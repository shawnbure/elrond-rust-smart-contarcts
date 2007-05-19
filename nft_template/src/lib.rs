#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait NftTemplate {
    #[init]
    fn init(
        &self,
        token_id: TokenIdentifier,
        royalties: Self::BigUint,
        base_uri: BoxedBytes,
        price: Self::BigUint,
    ) {
        self.token_id().set(&token_id);
        self.base_uri().set(&base_uri);
        self.royalties().set(&royalties);
        self.price().set(&price);
    }

    #[payable("EGLD")]
    #[endpoint(mintNft)]
    fn mint_nft(&self, #[payment_amount] payment: Self::BigUint) -> SCResult<()> {
        require!(payment >= self.price().get(), "Payment too low");

        let nonce = self.send().esdt_nft_create(
            &self.token_id().get(),
            &Self::BigUint::from(1u64),
            &BoxedBytes::empty(),
            &self.royalties().get(),
            &BoxedBytes::empty(),
            &BoxedBytes::empty(),
            &[self.base_uri().get()],
        );

        self.send().direct(
            &self.blockchain().get_caller(),
            &self.token_id().get(),
            nonce,
            &Self::BigUint::from(1u64),
            &[],
        );

        Ok(())
    }

    #[only_owner]
    #[endpoint(withdraw)]
    fn withdraw(&self) {
        self.send().direct_egld(
            &self.blockchain().get_caller(),
            &self
                .blockchain()
                .get_balance(&self.blockchain().get_sc_address()),
            &[],
        );
    }

    #[view(getPrice)]
    #[storage_mapper("price")]
    fn price(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[view(getRoyalties)]
    #[storage_mapper("royalties")]
    fn royalties(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[view(getBaseUri)]
    #[storage_mapper("base_uri")]
    fn base_uri(&self) -> SingleValueMapper<Self::Storage, BoxedBytes>;

    #[view(getTokenId)]
    #[storage_mapper("token_id")]
    fn token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;
}
