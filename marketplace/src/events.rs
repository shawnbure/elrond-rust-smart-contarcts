elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait EventsModule {
    #[allow(clippy::too_many_arguments)]
    #[event("put_nft_for_sale")]
    fn put_nft_for_sale_event(
        self,
        #[indexed] caller: Address,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] token_name: BoxedBytes,
        #[indexed] first_uri: BoxedBytes,
        #[indexed] last_uri: BoxedBytes,
        #[indexed] hash: BoxedBytes,
        #[indexed] attributes: BoxedBytes,
        #[indexed] price: Self::BigUint,
        #[indexed] royalties_percent: u64,
        #[indexed] timestamp: u64,
        #[indexed] tx_hash: H256,
    );

    #[allow(clippy::too_many_arguments)]
    #[event("buy_nft")]
    fn buy_nft_event(
        self,
        #[indexed] owner: Address,
        #[indexed] buyer: Address,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] price: Self::BigUint,
        #[indexed] timestamp: u64,
        #[indexed] tx_hash: H256,
    );

    #[event("withdraw_nft")]
    fn withdraw_nft_event(
        self,
        #[indexed] owner: Address,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] price: Self::BigUint,
        #[indexed] timestamp: u64,
        #[indexed] tx_hash: H256,
    );

    #[event("deposit_update")]
    fn deposit_update_event(&self, #[indexed] address: Address, #[indexed] amount: Self::BigUint);
}
