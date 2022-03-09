#![allow(clippy::too_many_arguments)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait EventsModule {
    #[event("put_nft_for_sale")]
    fn put_nft_for_sale_event(
        self,
        #[indexed] caller: ManagedAddress,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] token_name: ManagedBuffer,
        #[indexed] first_uri: ManagedBuffer,
        #[indexed] second_uri: ManagedBuffer,
        #[indexed] hash: ManagedBuffer,
        #[indexed] attributes: ManagedBuffer,
        #[indexed] price: BigUint,
        #[indexed] royalties_percent: u64,
        #[indexed] timestamp: u64,
        #[indexed] tx_hash: H256,
    );

    #[event("buy_nft")]
    fn buy_nft_event(
        self,
        #[indexed] owner: ManagedAddress,
        #[indexed] buyer: ManagedAddress,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] price: BigUint,
        #[indexed] timestamp: u64,
        #[indexed] tx_hash: H256,
    );

    #[event("withdraw_nft")]
    fn withdraw_nft_event(
        self,
        #[indexed] owner: ManagedAddress,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] price: BigUint,
        #[indexed] timestamp: u64,
        #[indexed] tx_hash: H256,
    );

    #[event("deposit_update")]
    fn deposit_update_event(&self, #[indexed] address: ManagedAddress, #[indexed] amount: BigUint);

    #[event("make_offer")]
    fn make_offer_event(
        &self,
        #[indexed] offeror: ManagedAddress,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] amount: BigUint,
        #[indexed] expire: u64,
        #[indexed] timestamp: u64,
        #[indexed] tx_hash: H256,
    );

    #[event("cancel_offer")]
    fn cancel_offer_event(
        &self,
        #[indexed] owner: ManagedAddress,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] amount: BigUint,
        #[indexed] timestamp: u64,
        #[indexed] tx_hash: H256,
    );

    #[event("accept_offer")]
    fn accept_offer_event(
        &self,
        #[indexed] owner: ManagedAddress,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] offeror: ManagedAddress,
        #[indexed] amount: BigUint,
        #[indexed] timestamp: u64,
        #[indexed] tx_hash: H256,
    );

    #[event("start_auction")]
    fn start_auction_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] token_name: ManagedBuffer,
        #[indexed] first_uri: ManagedBuffer,
        #[indexed] second_uri: ManagedBuffer,
        #[indexed] hash: ManagedBuffer,
        #[indexed] attributes: ManagedBuffer,
        #[indexed] min_bid: BigUint,
        #[indexed] start_time: u64,
        #[indexed] deadline: u64,
        #[indexed] royalties_percent: u64,
        #[indexed] timestamp: u64,
        #[indexed] tx_hash: H256,
    );

    #[event("place_bid")]
    fn place_bid_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] bid: BigUint,
        #[indexed] timestamp: u64,
        #[indexed] tx_hash: H256,
    );

    #[event("end_auction")]
    fn end_auction_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] winner: ManagedAddress,
        #[indexed] bid: BigUint,
        #[indexed] timestamp: u64,
        #[indexed] tx_hash: H256,
    );
}
