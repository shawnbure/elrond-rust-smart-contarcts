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
        max_supply: u64,
    ) {
        self.token_id().set(&token_id);
        self.base_uri().set(&base_uri);
        self.royalties().set(&royalties);
        self.price().set(&price);
        self.max_supply().set(&max_supply)
    }

    #[payable("EGLD")]
    #[endpoint(mintTokens)]
    fn mint_tokens(
        &self,
        #[payment_amount] payment: Self::BigUint,
        number_of_tokens_desired: u64,
    ) -> SCResult<()> {
        require!(number_of_tokens_desired > 0, "cannot mint zero tokens");

        let tokens_left_for_sale = self.get_left_for_sale();
        require!(tokens_left_for_sale > 0, "no tokens left for sale");

        let tokens_to_sell = core::cmp::min(tokens_left_for_sale, number_of_tokens_desired);
        let price_for_tokens_to_sell = self.price().get() * tokens_to_sell.into();
        require!(payment > price_for_tokens_to_sell, "payment too low");

        let uris = [self.base_uri().get()];
        let empty_box = BoxedBytes::empty();
        let token_id = self.token_id().get();
        let royalties = self.royalties().get();
        let big_one = Self::BigUint::from(1u64);
        let caller = self.blockchain().get_caller();

        for _ in 0..tokens_to_sell {
            let nonce = self.send().esdt_nft_create(
                &token_id,
                &big_one,
                &empty_box,
                &royalties,
                &empty_box,
                &empty_box,
                &uris[..],
            );
            self.send().direct(&caller, &token_id, nonce, &big_one, &[]);
        }

        let surplus = payment - price_for_tokens_to_sell;
        self.safe_send_egld(&caller, &surplus);

        self.total_sold().update(|x| *x += tokens_to_sell);
        Ok(())
    }

    fn safe_send_egld(&self, to: &Address, amount: &Self::BigUint) {
        if amount > &0 {
            self.send().direct_egld(to, amount, &[]);
        }
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

    #[view(getLeftForSale)]
    fn get_left_for_sale(&self) -> u64 {
        self.max_supply().get() - self.total_sold().get()
    }

    #[view(getTotalSold)]
    #[storage_mapper("total_sold")]
    fn total_sold(&self) -> SingleValueMapper<Self::Storage, u64>;

    #[view(getMaxSupply)]
    #[storage_mapper("max_supply")]
    fn max_supply(&self) -> SingleValueMapper<Self::Storage, u64>;

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
