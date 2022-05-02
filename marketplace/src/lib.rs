#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use core::ops::Deref;


pub mod config;
pub mod deposit;
pub mod events;
pub mod global_op;
pub mod royalties;
pub mod storage;
pub mod utils;
pub mod validation;

use storage::{AuctionInfo, NftId, NftSaleInfo, Offer, StakedPool, StakedAddressNFTs, StakedNFT};
const SECONDS_IN_YEARS: u64 = 31_556_952u64;

//const LAST_WITHDRAW_DATETIME_INIT: u64 = 0;

#[elrond_wasm::contract]
pub trait MarketplaceContract:
    events::EventsModule
    + storage::StorageModule
    + validation::ValidationModule
    + config::ConfigModule
    + utils::UtilsModule
    + global_op::GlobalOperationModule
    + deposit::DepositModule
    + royalties::RoyaltiesModule
{
    #[init]
    fn init(
        &self,
        platform_fee_percent: u64,
        royalties_max_fee_percent: u64,
        asset_min_price: BigUint,
        asset_max_price: BigUint,
        creator_withdrawal_waiting_epochs: u64,
        dao_address: ManagedAddress,
        version: ManagedBuffer,
    ) -> SCResult<()> {

        //create the staked pool if it does not exist
        if self.staked_pool().is_empty() {
            
            self.staked_pool().set(StakedPool::new(Vec::new()));
        }  
                
        self.version().set(&version);
        self.dao().set(&dao_address);
        self.try_set_platform_fee_percent(platform_fee_percent)?;
        self.try_set_royalties_max_fee_percent(royalties_max_fee_percent)?;
        self.try_set_asset_price_range(asset_min_price, asset_max_price)?;
        self.try_set_creator_withdrawal_waiting_epochs(creator_withdrawal_waiting_epochs)        
    }




  

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(createStakedPool)]
    fn create_staked_pool( &self )
    {
        //let mut vec = Vec::new();


        if self.staked_pool().is_empty() {
            //let mut vectorStakedAddressNFTIDs = Vec::new();

            //let stakedPool = StakedPool::new(Vec::new()<M>);
            //self.staked_pool().set(stakedPool); 

            self.staked_pool().set(StakedPool::new(Vec::new()));

        }   
        
        
    }
   


    #[payable("EGLD")]
    #[endpoint(depositStaking)]
    fn deposit_staking( &self, 
                        amount: BigUint ) -> SCResult<usize>
    {
       
        //caller address (since minting_limit is based on address)
        let caller_address = &self.blockchain().get_caller();

        let mut vec = Vec::new();
        vec.push(caller_address);
        vec.push(caller_address);


        Ok(vec.len())        
      
    }


   
    fn addAddressNFT(&self,
        address: ManagedAddress,
        token_id: TokenIdentifier,
        nonce: u64,)
    {
        //get the staked pool
        let stakedPool = self.staked_pool().get();

        //get the array of stakedAddressNFTs
        let arrayStakedAddressNFTs = stakedPool.arrayStakedAddressNFTs;


        let mut isStakedAddressNFTsFound = false;

        //iterate over the array of StakedAddressNFTs to see if address is in there already
        for stakedAddressNFTs in arrayStakedAddressNFTs 
        {
            //check if address already exist in stakedAddress
            if stakedAddressNFTs.address == address  
            {
                isStakedAddressNFTsFound = true;

                //now check to see if NFT is in arrayStakedNFTs
                
                let mut isStakedNFTFound = false;

                //iterate over the array of StakeNFTs if it's been staked already
                //let mut array2 = stakedAddressNFTs.arrayStakedNFTs;

                for stakedNFT in stakedAddressNFTs.arrayStakedNFTs
                {
                    if stakedNFT.token_id == token_id && stakedNFT.nonce == nonce
                    {
                        isStakedNFTFound = true;
                        break;
                    }
                }

                //if not staked, then add it 
                if ! isStakedNFTFound
                {
                    let stakedDateTime = self.blockchain().get_block_timestamp();

                    let newStakedNFT = StakedNFT::new(token_id.clone(), nonce, stakedDateTime);
        
                    

                    //stakedAddressNFTs.arrayStakedNFTs.push(newStakedNFT);
                }

                break;
            }
        }


        if ! isStakedAddressNFTsFound
        {
            //create new address

            let payoutInit = BigUint::zero(); 
            let stakedDateTime = self.blockchain().get_block_timestamp();

            let stakedNFT = StakedNFT::new(token_id.clone(), nonce, stakedDateTime);

            let mut arrayStakedNFTs = Vec::new();
            arrayStakedNFTs.push(stakedNFT);


            let stakedAddressNFTs = StakedAddressNFTs::new(address.clone(), arrayStakedNFTs, payoutInit, 0u64);

            //let arrayStakedAddressNFTsMod = stakedPool.arrayStakedAddressNFTs;
            //arrayStakedAddressNFTsMod.push(stakedAddressNFTs);
        }

        //iterate over the address to see if it exists
        // - if doesn't exist, then create StakedAddressNFTs, then add NFT to vector
        // - if exist, check if the NFT exist in vectorNFTIDs (for duplicate cases)
        //      - if it doesn't exist, create

        /*

        //add NFT (tokenID + nonce) to address

        let payoutInit = BigUint::zero(); 

        //let big_one = BigUint::from(1u64);
        //let big_zero = BigUint::zero();

        let timestamp = self.blockchain().get_block_timestamp();
        
        let stakedNFT1 = StakedNFT::new(token_id.clone(), nonce, timestamp);
        let stakedNFT2 = StakedNFT::new(token_id.clone(), nonce, timestamp);

        let mut vec = Vec::new();
        vec.push(stakedNFT1);
        vec.push(stakedNFT2);


       let stakedAddressNFTs_1 = StakedAddressNFTs::new(address, vec, payoutInit);
       //let stakedAddressNFTs_2 = StakedAddressNFTs::new(address, vec, payoutInit); 
        */
    }
   






    #[payable("*")]
    #[endpoint(putNftForSale)]
    fn put_nft_for_sale(
        &self,
        #[payment_token] token_id: TokenIdentifier,
        #[payment_nonce] nonce: u64,
        #[payment_amount] amount: BigUint,
        price: BigUint,
    ) -> SCResult<()> {
        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;
        self.require_valid_nft_amount(&amount)?;
        self.require_valid_price(&price)?;

        let token_data = self.get_token_data(&token_id, nonce);
        self.require_valid_royalties(&token_data)?;
        self.require_uris_not_empty(&token_data)?;

        let nft_id = NftId::new(token_id.clone(), nonce);
        self.require_nft_not_for_sale(&nft_id)?;
        self.require_nft_not_on_auction(&nft_id)?;

        let caller = self.blockchain().get_caller();
        let timestamp = self.blockchain().get_block_timestamp();
        let nft_sale_info = NftSaleInfo::new(caller.clone(), price.clone(), timestamp);

        self.nft_sale_info(&nft_id).set(&nft_sale_info);
        let tx_hash = self.blockchain().get_tx_hash();
        let mut uri_1 = ManagedBuffer::new();
        if token_data.uris.len() > 1 {
            let valid_uri = token_data.uris.get(1).deref().is_empty();
            if valid_uri {
                uri_1 = token_data.uris.get(1).deref().clone();
            }
        }

        self.put_nft_for_sale_event(
            caller,
            token_id,
            nonce,
            token_data.name,
            token_data.uris.get(0).deref().clone(),
            uri_1,
            token_data.hash,
            token_data.attributes,
            price,
            token_data.royalties.to_u64().unwrap(),
            timestamp,
            tx_hash,
        );

        Ok(())
    }

    #[payable("EGLD")]
    #[endpoint(buyNft)]
    fn buy_nft(
        &self,
        #[payment_amount] payment: BigUint,
        token_id: TokenIdentifier,
        nonce: u64,
    ) -> SCResult<()> {
        let payment_token: EsdtTokenPayment<Self::Api> = self.call_value().payment();

        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;

        let nft_id = NftId::new(token_id.clone(), nonce);
        self.require_nft_for_sale(&nft_id)?;

        let caller = self.blockchain().get_caller();
        let nft_sale_info = self.nft_sale_info(&nft_id).get();
        self.require_not_owns_nft(&caller, &nft_sale_info)?;

        let price = nft_sale_info.price;
        // self.try_increase_decrease_deposit(&caller, &payment, price)?;
        require!(payment == price, "not right amount of payment");

        let token_data = self.get_token_data(&token_id, nonce);
        let creator_cut = self.get_creator_cut(&payment, &token_data);
        self.set_creator_last_withdrawal_epoch_if_empty(&token_data.creator);
        // self.increase_creator_royalties(&token_data.creator, &creator_cut);

        let _amount_left = self.send().sell_nft(
            &token_id,
            nonce,
            &1u64.into(),
            &caller,
            &payment_token.token_identifier,
            payment_token.token_nonce,
            &(&creator_cut * &10u64.into()),
        );
        
        let platform_cut = self.get_platform_cut(&payment);
        // self.increase_platform_royalties(&platform_cut);
        self.send()
            .direct_egld(&self.dao().get(), &platform_cut, b"DAO's Cut");

        let nft_owner_cut = &payment - &platform_cut - &creator_cut;
        // self.increase_deposit(&nft_sale_info.owner, &nft_owner_cut);
        self.send()
            .direct_egld(&nft_sale_info.owner, &nft_owner_cut, b"Seller's Cut");

        // self.send()
        //     .direct_egld(&token_data.creator, &creator_cut, b"Creator's Cut");

        // self.send_nft(&caller, &token_id, nonce);
        self.nft_sale_info(&nft_id).clear();

        let timestamp = self.blockchain().get_block_timestamp();
        let tx_hash = self.blockchain().get_tx_hash();
        self.buy_nft_event(
            nft_sale_info.owner,
            caller,
            token_id,
            nonce,
            payment,
            timestamp,
            tx_hash,
        );
        Ok(())
    }

    #[endpoint(withdrawNft)]
    fn withdraw_nft(&self, token_id: TokenIdentifier, nonce: u64) -> SCResult<()> {
        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;

        let nft_id = NftId::new(token_id.clone(), nonce);
        if self.is_nft_for_sale(&nft_id) {
            self.withdraw_nft_from_sale(token_id, nonce)
        } else if self.is_nft_on_auction(&nft_id) {
            self.withdraw_nft_from_auction(token_id, nonce)
        } else {
            self.error_nft_not_found()
        }
    }

    fn withdraw_nft_from_sale(&self, token_id: TokenIdentifier, nonce: u64) -> SCResult<()> {
        let nft_id = NftId::new(token_id.clone(), nonce);
        let caller = self.blockchain().get_caller();
        let nft_sale_info = self.nft_sale_info(&nft_id).get();
        self.require_owns_nft(&caller, &nft_sale_info)?;

        self.send_nft(&caller, &token_id, nonce);
        self.nft_sale_info(&nft_id).clear();

        let timestamp = self.blockchain().get_block_timestamp();
        let tx_hash = self.blockchain().get_tx_hash();
        self.withdraw_nft_event(
            caller,
            token_id,
            nonce,
            nft_sale_info.price,
            timestamp,
            tx_hash,
        );

        Ok(())
    }

    fn withdraw_nft_from_auction(&self, token_id: TokenIdentifier, nonce: u64) -> SCResult<()> {
        let nft_id = NftId::new(token_id.clone(), nonce);
        let caller = self.blockchain().get_caller();
        let auction_info = self.auction(&nft_id).get();
        self.require_auction_owner(&caller, &auction_info)?;

        let timestamp = self.blockchain().get_block_timestamp();
        let deadline_passed = timestamp > auction_info.deadline;
        let auction_has_winner = auction_info.highest_bidder != ManagedAddress::zero();
        require!(
            !(deadline_passed && auction_has_winner),
            "auction has a winner"
        );

        let auction = self.auction(&nft_id).get();
        self.increase_deposit(&auction.highest_bidder, &auction.bid);

        self.send_nft(&caller, &token_id, nonce);
        self.auction(&nft_id).clear();

        let tx_hash = self.blockchain().get_tx_hash();
        self.withdraw_nft_event(
            caller,
            token_id,
            nonce,
            auction_info.min_bid,
            timestamp,
            tx_hash,
        );

        Ok(())
    }

    #[endpoint(makeOffer)]
    fn make_offers(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
        #[var_args] expire_opt: OptionalValue<u64>,
    ) -> SCResult<()> {
        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;
        self.require_valid_price(&amount)?;

        let expire = expire_opt
            .into_option()
            .unwrap_or(self.blockchain().get_block_timestamp() + SECONDS_IN_YEARS);
        self.require_valid_expire(expire)?;

        let caller = self.blockchain().get_caller();
        self.require_has_amount_in_deposit(&caller, &amount)?;

        let nft_id = NftId::new(token_id.clone(), nonce);
        if self.is_nft_for_sale(&nft_id) {
            self.make_offer_for_nft_on_sale(token_id, nonce, amount, expire)
        } else if self.is_nft_on_auction(&nft_id) {
            self.make_offer_for_nft_on_auction(token_id, nonce, amount, expire)
        } else {
            self.error_nft_not_found()
        }
    }

    fn make_offer_for_nft_on_sale(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
        expire: u64,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let nft_id = NftId::new(token_id.clone(), nonce);
        let nft_sale_info = self.nft_sale_info(&nft_id).get();
        self.require_not_owns_nft(&caller, &nft_sale_info)?;

        let list_timestamp = nft_sale_info.timestamp;
        let offer = Offer::new(amount.clone(), expire);
        self.offers(&caller, &nft_id, list_timestamp).set(&offer);

        let timestamp = self.blockchain().get_block_timestamp();
        let tx_hash = self.blockchain().get_tx_hash();
        self.make_offer_event(caller, token_id, nonce, amount, expire, timestamp, tx_hash);
        Ok(())
    }

    fn make_offer_for_nft_on_auction(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
        expire: u64,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let nft_id = NftId::new(token_id.clone(), nonce);
        let auction_info = self.auction(&nft_id).get();
        self.require_not_auction_owner(&caller, &auction_info)?;

        let list_timestamp = auction_info.created_at;
        let offer = Offer::new(amount.clone(), expire);
        self.offers(&caller, &nft_id, list_timestamp).set(&offer);

        let timestamp = self.blockchain().get_block_timestamp();
        let tx_hash = self.blockchain().get_tx_hash();
        self.make_offer_event(caller, token_id, nonce, amount, expire, timestamp, tx_hash);
        Ok(())
    }

    #[endpoint(acceptOffer)]
    fn accept_offers(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        offeror: ManagedAddress,
        amount: BigUint,
    ) -> SCResult<()> {
        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;
        self.require_valid_price(&amount)?;

        let nft_id = NftId::new(token_id.clone(), nonce);
        if self.is_nft_for_sale(&nft_id) {
            self.accept_offer_for_nft_on_sale(token_id, nonce, offeror, amount)
        } else if self.is_nft_on_auction(&nft_id) {
            self.accept_offer_for_nft_on_auction(token_id, nonce, offeror, amount)
        } else {
            self.error_nft_not_found()
        }
    }

    fn accept_offer_for_nft_on_sale(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        offeror: ManagedAddress,
        amount: BigUint,
    ) -> SCResult<()> {
        let nft_id = NftId::new(token_id.clone(), nonce);
        let caller = self.blockchain().get_caller();
        let nft_sale_info = self.nft_sale_info(&nft_id).get();
        self.require_owns_nft(&caller, &nft_sale_info)?;
        self.require_not_owns_nft(&offeror, &nft_sale_info)?;

        let list_timestamp = nft_sale_info.timestamp;
        self.require_offer_exists(&offeror, &nft_id, list_timestamp)?;
        let offer = self.offers(&offeror, &nft_id, list_timestamp).get();
        self.require_not_expired(&offer)?;
        let offer_amount = offer.amount;

        self.require_same_amounts(&amount, &offer_amount)?;
        self.try_decrease_deposit(&offeror, &offer_amount)?;

        let token_data = self.get_token_data(&token_id, nonce);
        let creator_cut = self.get_creator_cut(&offer_amount, &token_data);
        self.set_creator_last_withdrawal_epoch_if_empty(&token_data.creator);
        self.increase_creator_royalties(&token_data.creator, &creator_cut);

        let platform_cut = self.get_platform_cut(&offer_amount);
        self.increase_platform_royalties(&platform_cut);

        let nft_owner_cut = &offer_amount - &platform_cut - creator_cut;
        self.increase_deposit(&nft_sale_info.owner, &nft_owner_cut);

        self.send_nft(&offeror, &token_id, nonce);
        self.nft_sale_info(&nft_id).clear();
        self.offers(&offeror, &nft_id, list_timestamp).clear();

        let timestamp = self.blockchain().get_block_timestamp();
        let tx_hash = self.blockchain().get_tx_hash();
        self.accept_offer_event(
            caller,
            token_id,
            nonce,
            offeror,
            offer_amount,
            timestamp,
            tx_hash,
        );
        Ok(())
    }

    fn accept_offer_for_nft_on_auction(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        offeror: ManagedAddress,
        amount: BigUint,
    ) -> SCResult<()> {
        let nft_id = NftId::new(token_id.clone(), nonce);
        let caller = self.blockchain().get_caller();
        let auction_info = self.auction(&nft_id).get();
        self.require_auction_owner(&caller, &auction_info)?;
        self.require_not_auction_owner(&offeror, &auction_info)?;

        let timestamp = self.blockchain().get_block_timestamp();
        let auction_not_started = timestamp < auction_info.start_time;
        let deadline_passed = timestamp > auction_info.deadline;
        let has_no_winner = auction_info.highest_bidder == ManagedAddress::zero();
        let deadline_passed_and_has_no_winner = deadline_passed && has_no_winner;
        require!(
            auction_not_started || deadline_passed_and_has_no_winner,
            "auction ongoing or ended with a winner"
        );

        let list_timestamp = auction_info.created_at;
        self.require_offer_exists(&offeror, &nft_id, list_timestamp)?;
        let offer = self.offers(&offeror, &nft_id, list_timestamp).get();
        self.require_not_expired(&offer)?;
        let offer_amount = offer.amount;

        self.require_same_amounts(&amount, &offer_amount)?;
        self.try_decrease_deposit(&offeror, &offer_amount)?;

        let token_data = self.get_token_data(&token_id, nonce);
        let creator_cut = self.get_creator_cut(&offer_amount, &token_data);
        self.set_creator_last_withdrawal_epoch_if_empty(&token_data.creator);
        self.increase_creator_royalties(&token_data.creator, &creator_cut);

        let platform_cut = self.get_platform_cut(&offer_amount);
        self.increase_platform_royalties(&platform_cut);

        let nft_owner_cut = &offer_amount - &platform_cut - creator_cut;
        self.increase_deposit(&auction_info.owner, &nft_owner_cut);

        self.send_nft(&offeror, &token_id, nonce);
        self.auction(&nft_id).clear();
        self.offers(&offeror, &nft_id, list_timestamp).clear();

        let tx_hash = self.blockchain().get_tx_hash();
        self.accept_offer_event(
            caller,
            token_id,
            nonce,
            offeror,
            offer_amount,
            timestamp,
            tx_hash,
        );
        Ok(())
    }

    #[endpoint(cancelOffer)]
    fn cancel_offers(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
    ) -> SCResult<()> {
        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;
        self.require_valid_price(&amount)?;

        let nft_id = NftId::new(token_id.clone(), nonce);
        if self.is_nft_for_sale(&nft_id) {
            self.cancel_offer_for_nft_on_sale(token_id, nonce, amount)
        } else if self.is_nft_on_auction(&nft_id) {
            self.cancel_offer_for_nft_on_auction(token_id, nonce, amount)
        } else {
            self.error_nft_not_found()
        }
    }

    fn cancel_offer_for_nft_on_sale(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let nft_id = NftId::new(token_id.clone(), nonce);
        let nft_sale_info = self.nft_sale_info(&nft_id).get();

        let list_timestamp = nft_sale_info.timestamp;
        self.require_offer_exists(&caller, &nft_id, list_timestamp)?;
        self.offers(&caller, &nft_id, list_timestamp).clear();

        let timestamp = self.blockchain().get_block_timestamp();
        let tx_hash = self.blockchain().get_tx_hash();
        self.cancel_offer_event(caller, token_id, nonce, amount, timestamp, tx_hash);
        Ok(())
    }

    fn cancel_offer_for_nft_on_auction(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let nft_id = NftId::new(token_id.clone(), nonce);
        let nft_sale_info = self.auction(&nft_id).get();

        let list_timestamp = nft_sale_info.created_at;
        self.require_offer_exists(&caller, &nft_id, list_timestamp)?;
        self.offers(&caller, &nft_id, list_timestamp).clear();

        let timestamp = self.blockchain().get_block_timestamp();
        let tx_hash = self.blockchain().get_tx_hash();
        self.cancel_offer_event(caller, token_id, nonce, amount, timestamp, tx_hash);
        Ok(())
    }

    #[payable("*")]
    #[endpoint(startAuction)]
    fn start_auction(
        &self,
        #[payment_token] token_id: TokenIdentifier,
        #[payment_nonce] nonce: u64,
        #[payment_amount] amount: BigUint,
        min_bid: BigUint,
        deadline: u64,
        #[var_args] opt_start_time: OptionalValue<u64>,
    ) -> SCResult<()> {
        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;
        self.require_valid_nft_amount(&amount)?;
        self.require_valid_price(&min_bid)?;

        let nft_id = NftId::new(token_id.clone(), nonce);
        self.require_nft_not_on_auction(&nft_id)?;
        self.require_nft_not_for_sale(&nft_id)?;

        let timestamp = self.blockchain().get_block_timestamp();
        let start_time = opt_start_time.into_option().unwrap_or(timestamp);
        self.require_valid_deadline(deadline, start_time, timestamp)?;

        let token_data = self.get_token_data(&token_id, nonce);
        self.require_valid_royalties(&token_data)?;
        self.require_uris_not_empty(&token_data)?;

        let caller = self.blockchain().get_caller();
        let auction = AuctionInfo::new(
            caller.clone(),
            min_bid.clone(),
            start_time,
            deadline,
            timestamp,
            ManagedAddress::zero(),
            BigUint::zero(),
        );
        self.auction(&nft_id).set(&auction);

        let valid_uri = token_data.uris.get(1).deref().is_empty();
        let mut uri_1 = ManagedBuffer::new();
        if valid_uri {
            uri_1 = token_data.uris.get(1).deref().clone();
        }

        self.start_auction_event(
            caller,
            token_id,
            nonce,
            token_data.name,
            token_data.uris.get(0).deref().clone(),
            uri_1,
            token_data.hash,
            token_data.attributes,
            min_bid,
            start_time,
            deadline,
            token_data.royalties.to_u64().unwrap(),
            timestamp,
            self.blockchain().get_tx_hash(),
        );
        Ok(())
    }

    #[payable("EGLD")]
    #[endpoint(placeBid)]
    fn place_bid(
        &self,
        #[payment_amount] payment: BigUint,
        token_id: TokenIdentifier,
        nonce: u64,
        bid_amount: BigUint,
    ) -> SCResult<()> {
        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;
        self.require_valid_price(&bid_amount)?;

        let nft_id = NftId::new(token_id.clone(), nonce);
        self.require_nft_on_auction(&nft_id)?;

        let caller = self.blockchain().get_caller();
        let mut auction_info = self.auction(&nft_id).get();
        self.require_not_auction_owner(&caller, &auction_info)?;
        self.require_is_auction_ongoing(&auction_info)?;
        self.require_valid_new_bid(&bid_amount, &auction_info)?;

        self.increase_deposit(&auction_info.highest_bidder, &auction_info.bid);
        self.try_increase_decrease_deposit(&caller, &payment, &bid_amount)?;

        auction_info.highest_bidder = caller.clone();
        auction_info.bid = bid_amount.clone();
        self.auction(&nft_id).set(&auction_info);

        let timestamp = self.blockchain().get_block_timestamp();
        let tx_hash = self.blockchain().get_tx_hash();
        self.place_bid_event(caller, token_id, nonce, bid_amount, timestamp, tx_hash);
        Ok(())
    }

    #[endpoint(endAuction)]
    fn end_auction(&self, token_id: TokenIdentifier, nonce: u64) -> SCResult<()> {
        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;

        let nft_id = NftId::new(token_id.clone(), nonce);
        self.require_nft_on_auction(&nft_id)?;

        let auction_info = self.auction(&nft_id).get();
        self.require_deadline_passed(&auction_info)?;
        self.require_auction_has_winner(&auction_info)?;

        let caller = self.blockchain().get_caller();
        self.require_owner_or_winner(&caller, &auction_info)?;

        //Winner funds are already substracted at this point.
        let token_data = self.get_token_data(&token_id, nonce);
        let creator_cut = self.get_creator_cut(&auction_info.bid, &token_data);
        self.set_creator_last_withdrawal_epoch_if_empty(&token_data.creator);
        self.increase_creator_royalties(&token_data.creator, &creator_cut);

        let platform_cut = self.get_platform_cut(&auction_info.bid);
        self.increase_platform_royalties(&platform_cut);

        let nft_owner_cut = &auction_info.bid - &platform_cut - creator_cut;
        self.increase_deposit(&auction_info.owner, &nft_owner_cut);

        self.send_nft(&auction_info.highest_bidder, &token_id, nonce);
        self.auction(&nft_id).clear();

        let timestamp = self.blockchain().get_block_timestamp();
        let tx_hash = self.blockchain().get_tx_hash();
        self.end_auction_event(
            caller,
            token_id,
            nonce,
            auction_info.highest_bidder,
            auction_info.bid,
            timestamp,
            tx_hash,
        );
        Ok(())
    }

    #[view(getVersion)]
    #[storage_mapper("version")]
    fn version(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(getDao)]
    #[storage_mapper("dao")]
    fn dao(&self) -> SingleValueMapper<ManagedAddress>;
}
