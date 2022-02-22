#![no_std]
#![allow(clippy::too_many_arguments)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod random;
use random::Random;

const MAX_FEE_PERCENT: u64 = 10_000;
const PLATFORM_MINT_DEFAULT_FEE_PERCENT: u64 = 150;

const MIN_LOOP_ITERATION_GAS_LIMIT: u64 = 10_000_000;
const ERDSEA_ERD721_STANDARD: &[u8] = b"Erdsea|ERD-721";
const ERDSEA_WITHDRAW_MESSAGE: &[u8] = b"Erdsea website mint 1.5% fee";

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
        marketplace_admin: Address,
        token_id: TokenIdentifier,
        royalties: Self::BigUint,
        token_name_base: BoxedBytes,
        image_base_uri: BoxedBytes,
        image_extension: BoxedBytes,
        price: Self::BigUint,
        max_supply: u16,
        sale_start: u64,
        #[var_args] metadata_base_uri: OptionalArg<BoxedBytes>,
        #[var_args] admin_pub_key: OptionalArg<BoxedBytes>,
    ) {
        self.marketplace_admin().set(&marketplace_admin);
        self.token_id().set_if_empty(&token_id);
        self.royalties().set_if_empty(&royalties);
        self.token_name_base().set_if_empty(&token_name_base);
        self.image_base_uri().set(&image_base_uri);
        self.image_extension().set(&image_extension);
        self.metadata_base_uri().set_if_empty(
            &metadata_base_uri
                .into_option()
                .unwrap_or(BoxedBytes::empty()),
        );
        self.price().set_if_empty(&price);
        self.max_supply().set_if_empty(&max_supply);
        self.sale_start().set_if_empty(&sale_start);

        //set the admin_pub_key if provided in parameter
        self.admin_pub_key().set_if_empty(
            &admin_pub_key
                .into_option()
                .unwrap_or(BoxedBytes::empty()),
        );

        self.buyer_whitelist_check().set(&Self::BigInt::from(0));   //Default it to 0 which is OFF -> 1 is ON and 0 is OFF   
    }

    #[only_owner]
    #[endpoint(giveaway)]
    fn giveaway(
        &self,
        #[var_args] addr_amount_args: MultiArgVec<MultiArg2<Address, u16>>,
    ) -> SCResult<u16> {
        let token_name_base = self.token_name_base().get();
        let image_base_uri = self.image_base_uri().get();
        let image_extension = self.image_extension().get();
        let metadata_base_uri = self.metadata_base_uri().get();
        let empty_box = BoxedBytes::empty();
        let token_id = self.token_id().get();
        let royalties = self.royalties().get();
        let big_one = Self::BigUint::from(1u64);
        let big_zero = Self::BigUint::zero();
        let mut total_amount = 0u16;
        let mut next_expected_nonce = self.total_sold().get() + 1;

        for entry in addr_amount_args.into_vec() {
            let (address, amount) = entry.into_tuple();

            for _ in 0..amount {
                let nonce = self.send().esdt_nft_create(
                    &token_id,
                    &big_one,
                    &self.compute_token_name(&token_name_base, next_expected_nonce),
                    &royalties,
                    &empty_box,
                    &empty_box,
                    &self.compute_token_uris(
                        &image_base_uri,
                        &image_extension,
                        &metadata_base_uri,
                        next_expected_nonce,
                    ),
                );
                require!(nonce as u16 == next_expected_nonce, "unexpected nonce");
                next_expected_nonce += 1;

                self.send()
                    .direct(&address, &token_id, nonce, &big_one, &[]);
            }
            self.send()
                .direct_egld(&address, &big_zero, &ERDSEA_ERD721_STANDARD);

            total_amount += amount;
        }

        let tokens_left_for_sale = self.get_left_for_sale();
        require!(tokens_left_for_sale >= total_amount, "no tokens left");

        self.total_sold().update(|x| *x += total_amount);
        Ok(total_amount)
    }

    #[payable("EGLD")]
    #[endpoint(mintTokens)]
    fn mint_tokens_endpoint(
        &self,
        #[payment_amount] payment: Self::BigUint,
        #[var_args] number_of_tokens_desired_opt: OptionalArg<u16>,
    ) -> SCResult<()> {
        self.mint_tokens(payment, number_of_tokens_desired_opt)?;
        Ok(())
    }

    #[payable("EGLD")]
    #[endpoint(mintTokensThroughMarketplace)]
    fn mint_tokens_through_marketplace(
        &self,
        #[payment_amount] payment: Self::BigUint,
        number_of_tokens_desired: u16,
    ) -> SCResult<()> {

        //check if whitelist is enabled
        if self.is_buyer_whitelist_check_enabled() 
        {
            //====== Check if address is registered   ======
            if self.is_caller_address_not_part_of_whitelist() 
            {                          
                return sc_error!("Address is NOT part of WHITELIST");
            }    
            
            //====== Check address count > limit   ======            
            if self.check_buy_count_is_greater_than_buy_limit_by_adding_amount(number_of_tokens_desired) 
            {
                return sc_error!("Exceeded the Allowable Buy Limit for WhiteList"); 
            }  
            
        }


        

        require!(
            !self.minting_through_marketplace_denied().get(),
            "endpoint disabled"
        );

        


        //Verification of the signing 
        /*
        self.crypto().verify_ed25519
        let data = [token_id.as_esdt_identifier(), &nonce.to_be_bytes()].concat();
        let b_data = &data; 
        let u_data: &[u8] = &b_data;
        require!(
            self.crypto().verify_ed25519(
                self.admin_pub().get().as_slice(),
                u_data,
                signature.as_slice(),
            ) == true , "not verified"
        );
        */
        
        //verify against admin_pub_key
        
        let spent = self.mint_tokens(payment, elrond_wasm::types::OptionalArg::Some(number_of_tokens_desired))?;

        let marketplace_cut =
            &spent * &PLATFORM_MINT_DEFAULT_FEE_PERCENT.into() / MAX_FEE_PERCENT.into();
        self.marketplace_balance()
            .update(|x| *x += &marketplace_cut);

        
        //successfully minted so now we can add to the address count
        self.add_to_address_buy_count(number_of_tokens_desired);

        Ok(())
    }

    fn mint_tokens(
        &self,
        payment: Self::BigUint,
        number_of_tokens_desired_opt: OptionalArg<u16>,
    ) -> SCResult<Self::BigUint> {




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
        let image_extension = self.image_extension().get();
        let metadata_base_uri = self.metadata_base_uri().get();
        let empty_box = BoxedBytes::empty();
        let token_id = self.token_id().get();
        let royalties = self.royalties().get();
        let big_one = Self::BigUint::from(1u64);
        let caller = self.blockchain().get_caller();
        let mut next_expected_nonce = self.total_sold().get() + 1;

        for _ in 0..tokens_to_sell {
            let nonce = self.send().esdt_nft_create(
                &token_id,
                &big_one,
                &self.compute_token_name(&token_name_base, next_expected_nonce),
                &royalties,
                &empty_box,
                &empty_box,
                &self.compute_token_uris(
                    &image_base_uri,
                    &image_extension,
                    &metadata_base_uri,
                    next_expected_nonce,
                ),
            );
            require!(nonce as u16 == next_expected_nonce, "unexpected nonce");
            next_expected_nonce += 1;

            self.send().direct(&caller, &token_id, nonce, &big_one, &[]);
        }

        let surplus = &payment - &price_for_tokens_to_sell;
        self.send()
            .direct_egld(&caller, &surplus, &ERDSEA_ERD721_STANDARD);

        self.total_sold().update(|x| *x += tokens_to_sell);
        Ok(price_for_tokens_to_sell)
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
        image_extension: &BoxedBytes,
        metadata_base_uri: &BoxedBytes,
        expected_nonce: u16,
    ) -> Vec<BoxedBytes> {
        let mut result = Vec::new();
        let delimiter = BoxedBytes::from(&b"/"[..]);
        let index = self.get_token_index(expected_nonce);
        let index_string = self.u16_to_string(index);

        let own_image_uri = BoxedBytes::from_concat(&[
            image_base_uri.as_slice(),
            delimiter.as_slice(),
            index_string.as_slice(),
            image_extension.as_slice(),
        ]);
        result.push(own_image_uri);

        if !metadata_base_uri.is_empty() {
            let own_metadata_uri = BoxedBytes::from_concat(&[
                metadata_base_uri.as_slice(),
                delimiter.as_slice(),
                index_string.as_slice(),
            ]);
            result.push(own_metadata_uri);
        }

        result
    }

    fn compute_token_name(&self, token_name_base: &BoxedBytes, expected_nonce: u16) -> BoxedBytes {
        let delimiter = BoxedBytes::from(&b" #"[..]);
        let expected_nonce_string = self.u16_to_string(expected_nonce);

        BoxedBytes::from_concat(&[
            token_name_base.as_slice(),
            delimiter.as_slice(),
            expected_nonce_string.as_slice(),
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

    #[only_owner]
    #[endpoint(requestWithdraw)]
    fn request_withdraw(&self, marketplace: Address) -> AsyncCall<Self::SendApi> {
        self.marketplace_proxy(marketplace)
            .withdraw_creator_royalties()
            .async_call()
    }

    #[only_owner]
    #[endpoint(withdraw)]
    fn withdraw(&self, #[var_args] amount_opt: OptionalArg<Self::BigUint>) {
        let amount = amount_opt.into_option().unwrap_or(
            self.blockchain()
                .get_sc_balance(&TokenIdentifier::egld(), 0)
                - self.marketplace_balance().get(),
        );

        let caller = self.blockchain().get_caller();
        self.send().direct_egld(&caller, &amount, &[]);
    }

    #[endpoint(marketplaceWithdraw)]
    fn marketplace_withdraw(
        &self,
        #[var_args] amount_opt: OptionalArg<Self::BigUint>,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        require!(
            caller == self.marketplace_admin().get(),
            "not markeplace admin"
        );

        let amount = amount_opt
            .into_option()
            .unwrap_or(self.marketplace_balance().get());
        self.marketplace_balance().update(|x| *x -= &amount);

        self.send()
            .direct_egld(&caller, &amount, ERDSEA_WITHDRAW_MESSAGE);
        Ok(())
    }

    #[only_owner]
    #[endpoint(allowMintingThroughMarketplace)]
    fn allow_minting_through_marketplace(&self) {
        self.minting_through_marketplace_denied().set(&false);
    }

    #[only_owner]
    #[endpoint(denyMintingThroughMarketplace)]
    fn deny_minting_through_marketplace(&self) {
        self.minting_through_marketplace_denied().set(&true);
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

    #[view(getMarketplaceBalance)]
    #[storage_mapper("marketplace_balance")]
    fn marketplace_balance(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[view(getMarketplaceAdmin)]
    #[storage_mapper("marketplace_admin")]
    fn marketplace_admin(&self) -> SingleValueMapper<Self::Storage, Address>;

    #[view(isMintingThroughMarketplaceDenied)]
    #[storage_mapper("minting_through_marketplace_denied")]
    fn minting_through_marketplace_denied(&self) -> SingleValueMapper<Self::Storage, bool>;

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

    #[view(getImageExtension)]
    #[storage_mapper("image_extension")]
    fn image_extension(&self) -> SingleValueMapper<Self::Storage, BoxedBytes>;

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

    #[proxy]
    fn marketplace_proxy(&self, to: Address) -> marketplace_proxy::Proxy<Self::SendApi>;

    
    #[view(getAdminPubKey)]
    #[storage_mapper("admin_pub_key")]
    fn admin_pub_key(&self) -> SingleValueMapper<Self::Storage, BoxedBytes>;    





   //===================================================================================================
    // WHITELIST - BUY COUNT / LIMIT
    //===================================================================================================

    //works 2/21
    #[view(getBuyCount)]
    #[storage_mapper("buy_count")]
    fn buy_count(&self, address: &Address) -> SingleValueMapper<Self::Storage, u16>;

    //works 2/21
    #[view(getBuyLimit)]
    #[storage_mapper("buy_limit")]
    fn buy_limit(&self, address: &Address) -> SingleValueMapper<Self::Storage, u16>;



    //works 2/21
    //CREATE MINTING COUNT & LIMIT (Used during population)
    //----------------------------------------------------------------------
    #[payable("EGLD")]
    #[only_owner]
    #[endpoint(createBuyerAddress)]
    fn create_buyer_address(&self,
                            buy_count: u16,
                            buy_limit: u16,
                            address: Address) -> SCResult<()>
    {          
        //ONLY Create new address record if it doesn't exist    
        if self.buy_limit(&address).is_empty() 
        {
            self.buy_count(&address).set(&buy_count);
            self.buy_limit(&address).set(&buy_limit);
        } 
        
        Ok(())
    }
    

    //working 2/21    
    // [PRIVATE] - Check to see if caller is not part of  whitelist by checking buy_limit (empty)
    //----------------------------------------------------------------------
    fn is_caller_address_not_part_of_whitelist(&self) -> bool
    {
        //caller address (since minting_limit is based on address)
        let caller_address = &self.blockchain().get_caller();  
        
        //check limit since it should be zero (at least 1 if created for address)
        return self.buy_limit(&caller_address).is_empty() 
    }


    /* FOR TESTING PURPOSE (LEAVE COMMENTED OUT)
    //TEST FUNC working 2/21
    #[payable("EGLD")]   //remove
    #[endpoint] //TODO REMOVE: remove after testing
    fn is_caller_address_not_part_of_whitelist2(&self) -> SCResult<(u64)>
    {
        //caller address (since minting_limit is based on address)
        let caller_address = &self.blockchain().get_caller();  

        if( self.buy_limit(&caller_address).is_empty() )
        {
            Ok(1)
        }
        else
        {
            Ok(0)
        }
    }
    */



    //working 2/21
    // [PRIVATE] - check if buy count < buy limit after adding to the buy count
    //----------------------------------------------------------------------   
    fn check_buy_count_is_greater_than_buy_limit_by_adding_amount(&self,
                                                                  amount_to_add_to_buy_count: u16) -> bool
    {
        //get caller buy limit
        let buy_limit = self.buy_limit(&self.blockchain().get_caller()).get();
        
        //get miting count for caller and add amount to it 
        let mut buy_count_mut = self.buy_count(&self.blockchain().get_caller()).get();
        buy_count_mut += amount_to_add_to_buy_count;

        //check if the "new" (new by adding amount to it) buy count is greater than buy limit
        return buy_count_mut > buy_limit;     
    }    


    /*
    //FOR TESTING PURPOSE (LEAVE COMMENTED OUT)
    #[payable("EGLD")]
    #[endpoint] 
    fn check_buy_count_is_greater_than_buy_limit_by_adding_amount2(&self,
                                                                  amount_to_add_to_buy_count: u16) -> SCResult<(u64)>
    {
        //check if the "new" (new by adding amount to it) buy count is greater than buy limit
        if self.check_buy_count_is_greater_than_buy_limit_by_adding_amount(amount_to_add_to_buy_count) 
        {
            Ok(1)
        }
        else
        {
            Ok(2)
        }
    }   
    */

    //working 2/21
    // [PRIVATE] - ADD TO MINTING COUNT BY BIGINT PARAM
    //----------------------------------------------------------------------
    //#[payable("EGLD")]   //remove
    //#[endpoint] //TODO REMOVE: remove after testing
    fn add_to_address_buy_count(&self,
                                amount: u16) -> SCResult<()>
    {
        let address = self.blockchain().get_caller();

        if self.buy_limit(&address).is_empty()  //check limit since limit is never 0 (empty)
        {
            return sc_error!("Address is NOT CREATED for Buying");
        }
        else
        {
            self.buy_count(&address).update(|buy_count| *buy_count += amount);

            Ok(()) //SUCCESS        
        }
    }



    //===================================================================================================
    // WHITELIST - BUYER MINTING CHECKS FLAGS
    //===================================================================================================

    //works: 2/21
    //1: ON and 0: OFF 
    #[view(getBuyerWhiteListCheck)]
    #[storage_mapper("buyer_whitelist_check")]
    fn buyer_whitelist_check(&self) -> SingleValueMapper<Self::Storage, Self::BigInt>;  


    //works 2/21
    // PRIVATE : CHECK "BUYER" WHITELIST CHECK is Enabled (PRIVATE)
    fn is_buyer_whitelist_check_enabled(&self) -> bool
    {
        return self.buyer_whitelist_check().get() == Self::BigInt::from(1);  //1 is ON and 0 is OFF
    }
    

    
    //works 2/21
    // [ENDPOINT] UPDATE "BUYER" WHITELIST CHECK 
    //----------------------------------------------------------------------  
    #[payable("EGLD")]  
    #[only_owner]
    #[endpoint(updateBuyerWhitelistCheck)]
    fn update_buyer_whitelist_check(&self,
                                    whitelist_check: Self::BigInt)
    {  
        //1: On and 0: Off
        self.buyer_whitelist_check().set(&whitelist_check);
    }



}
