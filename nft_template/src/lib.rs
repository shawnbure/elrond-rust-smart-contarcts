#![no_std]
#![allow(clippy::too_many_arguments)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod random;
use random::Random;

const MIN_LOOP_ITERATION_GAS_LIMIT: u64 = 10_000_000;

mod marketplace_proxy {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait MarketPlace {
        #[endpoint(withdrawCreatorRoyalties)]
        fn withdraw_creator_royalties(&self);
    }
}

#[elrond_wasm::contract]
pub trait NftTemplate {
    #[init]
    fn init(
        &self,
        token_id: TokenIdentifier,
        royalties: Self::BigUint,
        token_name_base: BoxedBytes,
        image_base_uri: BoxedBytes,
        metadata_base_uri: BoxedBytes,
        price: Self::BigUint,
        max_supply: u16,
        sale_start: u64,
    ) {
        self.token_id().set_if_empty(&token_id);
        self.royalties().set_if_empty(&royalties);
        self.token_name_base().set_if_empty(&token_name_base);
        self.image_base_uri().set(&image_base_uri);
        self.metadata_base_uri().set_if_empty(&metadata_base_uri);
        self.price().set_if_empty(&price);
        self.max_supply().set_if_empty(&max_supply);
        self.sale_start().set_if_empty(&sale_start);
    }

    #[only_owner]
    #[endpoint(giveaway)]
    fn giveaway(
        &self,
        #[var_args] addr_amount_args: MultiArgVec<MultiArg2<Address, u16>>,
    ) -> SCResult<u16> {
        let token_name_base = self.token_name_base().get();
        let image_base_uri = self.image_base_uri().get();
        let metadata_base_uri = self.metadata_base_uri().get();
        let empty_box = BoxedBytes::empty();
        let token_id = self.token_id().get();
        let royalties = self.royalties().get();
        let big_one = Self::BigUint::from(1u64);
        let mut total_amount = 0u16;
        let mut next_expected_nonce = self.total_sold().get() + 1;

        for entry in addr_amount_args.into_vec() {
            let (address, amount) = entry.into_tuple();

            for _ in 0..amount {
                let index = self.get_token_index(next_expected_nonce);
                let nonce = self.send().esdt_nft_create(
                    &token_id,
                    &big_one,
                    &self.compute_token_name(&token_name_base, index),
                    &royalties,
                    &empty_box,
                    &empty_box,
                    &self.compute_token_uris(&image_base_uri, &metadata_base_uri, index),
                );
                require!(nonce as u16 == next_expected_nonce, "unexpected nonce");
                next_expected_nonce += 1;

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
        #[var_args] number_of_tokens_desired_opt: OptionalArg<u16>,
    ) -> SCResult<()> {
        let current_timestamp = self.blockchain().get_block_timestamp();
        let sale_start_timestamp = self.sale_start().get();
        require!(
            current_timestamp >= sale_start_timestamp,
            "sale did not start"
        );
        require!(!self.sale_paused().get(), "sale is paused");

        let number_of_tokens_desired = number_of_tokens_desired_opt.into_option().unwrap_or(1u16);
        require!(number_of_tokens_desired > 0, "cannot mint zero tokens");

        let tokens_left_for_sale = self.get_left_for_sale();
        require!(tokens_left_for_sale > 0, "no tokens left for sale");

        let tokens_to_sell = core::cmp::min(tokens_left_for_sale, number_of_tokens_desired);
        let price_for_tokens_to_sell = self.price().get() * (tokens_to_sell as u64).into();
        require!(payment >= price_for_tokens_to_sell, "payment too low");

        let token_name_base = self.token_name_base().get();
        let image_base_uri = self.image_base_uri().get();
        let metadata_base_uri = self.metadata_base_uri().get();
        let empty_box = BoxedBytes::empty();
        let token_id = self.token_id().get();
        let royalties = self.royalties().get();
        let big_one = Self::BigUint::from(1u64);
        let caller = self.blockchain().get_caller();
        let mut next_expected_nonce = self.total_sold().get() + 1;

        for _ in 0..tokens_to_sell {
            let index = self.get_token_index(next_expected_nonce);
            let nonce = self.send().esdt_nft_create(
                &token_id,
                &big_one,
                &self.compute_token_name(&token_name_base, index),
                &royalties,
                &empty_box,
                &empty_box,
                &self.compute_token_uris(&image_base_uri, &metadata_base_uri, index),
            );
            require!(nonce as u16 == next_expected_nonce, "unexpected nonce");
            next_expected_nonce += 1;

            self.send().direct(&caller, &token_id, nonce, &big_one, &[]);
        }

        let surplus = payment - price_for_tokens_to_sell;
        self.safe_send_egld(&caller, &surplus);

        self.total_sold().update(|x| *x += tokens_to_sell);
        Ok(())
    }

    #[only_owner]
    #[endpoint]
    fn shuffle(&self) -> SCResult<u64> {
        let sale_start_timestamp = self.sale_start().get();
        let current_timestamp = self.blockchain().get_block_timestamp();
        require!(current_timestamp < sale_start_timestamp, "sale started");

        let mut random = Random::new(
            *self.blockchain().get_block_random_seed(),
            self.blockchain().get_tx_hash().as_bytes(),
        );
        let max_supply = self.max_supply().get() as u16;

        let mut iterations = 0;
        loop {
            let gas_left = self.blockchain().get_gas_left();

            if gas_left < MIN_LOOP_ITERATION_GAS_LIMIT {
                break;
            }

            let random1 = random.next_u16() % max_supply + 1;
            let random2 = random.next_u16() % max_supply + 1;

            let index1 = self.get_token_index(random1);
            let index2 = self.get_token_index(random2);

            self.nonce_to_index(index1).set(&index2);
            self.nonce_to_index(index2).set(&index1);

            iterations += 1;
        }

        Ok(iterations)
    }

    fn get_token_index(&self, nonce: u16) -> u16 {
        let index = self.nonce_to_index(nonce).get();
        if index == 0 {
            nonce
        } else {
            index
        }
    }

    fn compute_token_uris(
        &self,
        image_base_uri: &BoxedBytes,
        metadata_base_uri: &BoxedBytes,
        index: u16,
    ) -> Vec<BoxedBytes> {
        let mut result = Vec::new();
        let delimiter = BoxedBytes::from(&b"/"[..]);
        let index_string = self.u16_to_string(index);

        let own_image_uri = BoxedBytes::from_concat(&[
            image_base_uri.as_slice(),
            delimiter.as_slice(),
            index_string.as_slice(),
        ]);

        let own_metadata_uri = BoxedBytes::from_concat(&[
            metadata_base_uri.as_slice(),
            delimiter.as_slice(),
            index_string.as_slice(),
        ]);

        result.push(own_image_uri);
        result.push(own_metadata_uri);

        result
    }

    fn compute_token_name(&self, token_name_base: &BoxedBytes, index: u16) -> BoxedBytes {
        let delimiter = BoxedBytes::from(&b" #"[..]);
        let index_string = self.u16_to_string(index);

        BoxedBytes::from_concat(&[
            token_name_base.as_slice(),
            delimiter.as_slice(),
            index_string.as_slice(),
        ])
    }

    fn u16_to_string(&self, a: u16) -> BoxedBytes {
        let ascii_zero_char = 48;
        let mut vec = Vec::new();
        let mut num = a;

        loop {
            vec.push(ascii_zero_char + (num % 10) as u8);
            num /= 10;

            if num == 0 {
                break;
            }
        }

        vec.reverse();
        vec.as_slice().into()
    }

    fn safe_send_egld(&self, to: &Address, amount: &Self::BigUint) {
        if amount > &0 {
            self.send().direct_egld(to, amount, &[]);
        }
    }

    #[proxy]
    fn marketplace_proxy(&self, to: Address) -> marketplace_proxy::Proxy<Self::SendApi>;

    #[only_owner]
    #[endpoint(requestWithdraw)]
    fn request_withdraw(&self, marketplace: Address) {
        self.marketplace_proxy(marketplace)
            .withdraw_creator_royalties()
            .execute_on_dest_context_ignore_result();
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
    #[endpoint(setPrice)]
    fn set_price(&self, price: Self::BigUint) {
        self.price().set(&price);
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
    fn get_left_for_sale(&self) -> u16 {
        self.max_supply().get() - self.total_sold().get()
    }

    #[view(getMaxSupplyAndTotalSold)]
    fn get_max_supply_and_total_sold(&self) -> MultiResult2<u16, u16> {
        MultiResult2::from((self.max_supply().get(), self.total_sold().get()))
    }

    #[view(getTotalSold)]
    #[storage_mapper("total_sold")]
    fn total_sold(&self) -> SingleValueMapper<Self::Storage, u16>;

    #[view(getMaxSupply)]
    #[storage_mapper("max_supply")]
    fn max_supply(&self) -> SingleValueMapper<Self::Storage, u16>;

    #[view(getPrice)]
    #[storage_mapper("price")]
    fn price(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[view(getRoyalties)]
    #[storage_mapper("royalties")]
    fn royalties(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[view(getImageBaseUri)]
    #[storage_mapper("image_base_uri")]
    fn image_base_uri(&self) -> SingleValueMapper<Self::Storage, BoxedBytes>;

    #[view(getMetadataBaseUri)]
    #[storage_mapper("metadata_base_uri")]
    fn metadata_base_uri(&self) -> SingleValueMapper<Self::Storage, BoxedBytes>;

    #[view(getTokenNameBase)]
    #[storage_mapper("token_name_base")]
    fn token_name_base(&self) -> SingleValueMapper<Self::Storage, BoxedBytes>;

    #[view(getTokenId)]
    #[storage_mapper("token_id")]
    fn token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

    #[view(getSaleStart)]
    #[storage_mapper("sale_start")]
    fn sale_start(&self) -> SingleValueMapper<Self::Storage, u64>;

    #[view(isSalePaused)]
    #[storage_mapper("sale_paused")]
    fn sale_paused(&self) -> SingleValueMapper<Self::Storage, bool>;

    #[storage_mapper("nonce_to_index")]
    fn nonce_to_index(&self, nonce_as_u16: u16) -> SingleValueMapper<Self::Storage, u16>;
}
