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
        sale_start_timestamp: u64,
    ) {
        self.token_id().set_if_empty(&token_id);
        self.base_uri().set_if_empty(&base_uri);
        self.royalties().set_if_empty(&royalties);
        self.price().set_if_empty(&price);
        self.max_supply().set_if_empty(&max_supply);
        self.sale_start_timestamp()
            .set_if_empty(&sale_start_timestamp);
    }

    #[only_owner]
    #[endpoint(giveaway)]
    fn giveaway(
        &self,
        #[var_args] addr_amount_args: MultiArgVec<MultiArg2<Address, u64>>,
    ) -> SCResult<u64> {
        let uris = [self.base_uri().get()];
        let empty_box = BoxedBytes::empty();
        let token_id = self.token_id().get();
        let royalties = self.royalties().get();
        let big_one = Self::BigUint::from(1u64);
        let mut total_amount = 0u64;

        for entry in addr_amount_args.into_vec() {
            let (address, amount) = entry.into_tuple();

            for _ in 0..amount {
                let nonce = self.send().esdt_nft_create(
                    &token_id,
                    &big_one,
                    &empty_box,
                    &royalties,
                    &empty_box,
                    &empty_box,
                    &uris[..],
                );
                self.send()
                    .direct(&address, &token_id, nonce, &big_one, &[]);
            }

            total_amount += amount;
        }

        let tokens_left_for_sale = self.get_left_for_sale();
        require!(tokens_left_for_sale >= total_amount, "no tokens left");

        self.total_sold().update(|x| *x += total_amount);
        Ok(total_amount)
    }

    #[payable("EGLD")]
    #[endpoint(mintTokens)]
    fn mint_tokens(
        &self,
        #[payment_amount] payment: Self::BigUint,
        number_of_tokens_desired: u64,
    ) -> SCResult<()> {
        let current_timestamp = self.blockchain().get_block_timestamp();
        let sale_start_timestamp = self.sale_start_timestamp().get();
        require!(
            current_timestamp >= sale_start_timestamp,
            "sale did not start"
        );
        require!(!self.sale_paused().get(), "sale is paused");
        require!(number_of_tokens_desired > 0, "cannot mint zero tokens");

        let tokens_left_for_sale = self.get_left_for_sale();
        require!(tokens_left_for_sale > 0, "no tokens left for sale");

        let tokens_to_sell = core::cmp::min(tokens_left_for_sale, number_of_tokens_desired);
        let price_for_tokens_to_sell = self.price().get() * tokens_to_sell.into();
        require!(payment >= price_for_tokens_to_sell, "payment too low");

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

    #[only_owner]
    #[endpoint(pauseSale)]
    fn pause_sale(&self) {
        self.sale_paused().set(&true);
    }

    #[only_owner]
    #[endpoint(resumeSale)]
    fn resume_sale(&self) {
        self.sale_paused().set(&false);
    }

    #[view(getLeftForSale)]
    fn get_left_for_sale(&self) -> u64 {
        self.max_supply().get() - self.total_sold().get()
    }

    #[view(getMaxSupplyAndTotalSold)]
    fn get_max_supply_and_total_sold(&self) -> MultiResult2<u64, u64> {
        MultiResult2::from((self.max_supply().get(), self.total_sold().get()))
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

    #[view(getSaleStartTimestamp)]
    #[storage_mapper("sale_start_timestamp")]
    fn sale_start_timestamp(&self) -> SingleValueMapper<Self::Storage, u64>;

    #[view(isSalePaused)]
    #[storage_mapper("sale_paused")]
    fn sale_paused(&self) -> SingleValueMapper<Self::Storage, bool>;
}
