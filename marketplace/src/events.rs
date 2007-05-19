elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait EventsModule {
    #[event("put_nft_for_sale")]
    fn put_nft_for_sale_event(
        self,
        #[indexed] caller: Address,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] uri: BoxedBytes,
        #[indexed] collection: BoxedBytes,
        #[indexed] price: Self::BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("buy_nft")]
    fn buy_nft_event(
        self,
        #[indexed] owner: Address,
        #[indexed] buyer: Address,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] uri: BoxedBytes,
        #[indexed] collection: BoxedBytes,
        #[indexed] price: Self::BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("withdraw_nft")]
    fn withdraw_nft_event(
        self,
        #[indexed] owner: Address,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] uri: BoxedBytes,
        #[indexed] collection: BoxedBytes,
        #[indexed] price: Self::BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("collection_register")]
    fn collection_register_event(
        self,
        #[indexed] caller: Address,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] collection: BoxedBytes,
        #[indexed] timestamp: u64,
    );
}
