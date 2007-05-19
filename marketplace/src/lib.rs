#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod config;
pub mod deposit;
pub mod events;
pub mod global_op;
pub mod royalties;
pub mod storage;
pub mod utils;
pub mod validation;

use storage::{NftId, NftSaleInfo};

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
        asset_min_price: Self::BigUint,
        asset_max_price: Self::BigUint,
        creator_withdrawal_waiting_epochs: u64,
    ) -> SCResult<()> {
        self.try_set_platform_fee_percent(platform_fee_percent)?;
        self.try_set_royalties_max_fee_percent(royalties_max_fee_percent)?;
        self.try_set_asset_price_range(asset_min_price, asset_max_price)?;
        self.try_set_creator_withdrawal_waiting_epochs(creator_withdrawal_waiting_epochs)
    }

    #[payable("*")]
    #[endpoint(putNftForSale)]
    fn put_nft_for_sale(
        &self,
        #[payment_token] token_id: TokenIdentifier,
        #[payment_nonce] nonce: u64,
        #[payment_amount] amount: Self::BigUint,
        price: Self::BigUint,
    ) -> SCResult<()> {
        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;
        self.require_valid_amount(&amount)?;
        self.require_valid_price(&price)?;

        let token_data = self.get_token_data(&token_id, nonce);
        self.require_valid_royalties(&token_data)?;
        self.require_uris_not_empty(&token_data)?;

        let nft_id = NftId::new(token_id.clone(), nonce);
        self.require_nft_not_for_sale(&nft_id)?;

        let caller = self.blockchain().get_caller();
        let timestamp = self.blockchain().get_block_timestamp();
        let nft_sale_info = NftSaleInfo::new(caller.clone(), price.clone(), timestamp);

        self.nft_sale_info(&nft_id).set(&nft_sale_info);
        let tx_hash = self.blockchain().get_tx_hash();
        self.put_nft_for_sale_event(
            caller,
            token_id,
            nonce,
            token_data.name,
            token_data.uris.first().unwrap().clone(),
            token_data.uris.last().unwrap().clone(),
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
        #[payment_amount] payment: Self::BigUint,
        token_id: TokenIdentifier,
        nonce: u64,
    ) -> SCResult<()> {
        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;

        let nft_id = NftId::new(token_id.clone(), nonce);
        self.require_nft_for_sale(&nft_id)?;

        let caller = self.blockchain().get_caller();
        let nft_sale_info = self.nft_sale_info(&nft_id).get();
        self.require_not_owns_nft(&caller, &nft_sale_info)?;

        let price = &nft_sale_info.price;
        let caller_deposit = self.try_increase_decrease_deposit(&caller, &payment, price)?;

        let token_data = self.get_token_data(&token_id, nonce);
        let creator_cut = self.get_creator_cut(&payment, &token_data);
        self.set_creator_last_withdrawal_epoch_if_empty(&token_data.creator);
        self.increase_creator_royalties(&token_data.creator, &creator_cut);

        let platform_cut = self.get_platform_cut(&payment);
        self.increase_platform_royalties(&platform_cut);

        let nft_owner_cut = &payment - &platform_cut - creator_cut;
        let nft_owner_deposit = self.increate_deposit(&nft_sale_info.owner, &nft_owner_cut);

        self.send_nft(&caller, &token_id, nonce);
        self.nft_sale_info(&nft_id).clear();

        let timestamp = self.blockchain().get_block_timestamp();
        let tx_hash = self.blockchain().get_tx_hash();
        self.buy_nft_event(
            nft_sale_info.owner.clone(),
            caller.clone(),
            token_id,
            nonce,
            payment,
            timestamp,
            tx_hash.clone(),
        );
        self.deposit_update_event(caller, caller_deposit, timestamp, tx_hash.clone());
        self.deposit_update_event(nft_sale_info.owner, nft_owner_deposit, timestamp, tx_hash);
        Ok(())
    }

    #[endpoint(withdrawNft)]
    fn withdraw_nft(&self, token_id: TokenIdentifier, nonce: u64) -> SCResult<()> {
        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;

        let nft_id = NftId::new(token_id.clone(), nonce);
        self.require_nft_for_sale(&nft_id)?;

        let caller = self.blockchain().get_caller();
        let nft_sale_info = self.nft_sale_info(&nft_id).get();
        self.require_owns_nft(&caller, &nft_sale_info)?;

        self.send_nft(&caller, &token_id, nonce);

        let timestamp = self.blockchain().get_block_timestamp();
        self.nft_sale_info(&nft_id).clear();

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

    #[endpoint(makeOffer)]
    fn make_offer(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        amount: Self::BigUint,
    ) -> SCResult<()> {
        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;
        self.require_valid_price(&amount)?;

        let caller = self.blockchain().get_caller();
        self.require_has_amount_in_deposit(&caller, &amount)?;

        let nft_id = NftId::new(token_id.clone(), nonce);
        self.require_nft_for_sale(&nft_id)?;

        let nft_sale_info = self.nft_sale_info(&nft_id).get();
        self.require_not_owns_nft(&caller, &nft_sale_info)?;

        let list_timestamp = nft_sale_info.timestamp;
        self.offer(&caller, &nft_id, list_timestamp).set(&amount);

        let timestamp = self.blockchain().get_block_timestamp();
        let tx_hash = self.blockchain().get_tx_hash();
        self.make_offer_event(caller, token_id, nonce, amount, timestamp, tx_hash);
        Ok(())
    }

    #[endpoint(acceptOffer)]
    fn accept_offer(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        offeror: Address,
        amount: Self::BigUint,
    ) -> SCResult<()> {
        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_nonce(nonce)?;
        self.require_valid_price(&amount)?;

        let nft_id = NftId::new(token_id.clone(), nonce);
        self.require_nft_for_sale(&nft_id)?;

        let caller = self.blockchain().get_caller();
        let nft_sale_info = self.nft_sale_info(&nft_id).get();
        self.require_owns_nft(&caller, &nft_sale_info)?;
        self.require_not_owns_nft(&offeror, &nft_sale_info)?;

        let list_timestamp = nft_sale_info.timestamp;
        self.require_offer_exists(&offeror, &nft_id, list_timestamp)?;
        let offer_amount = self.offer(&offeror, &nft_id, list_timestamp).get();
        self.require_same_amounts(&amount, &offer_amount)?;
        let offeror_deposit = self.try_decrease_deposit(&offeror, &offer_amount)?;

        let token_data = self.get_token_data(&token_id, nonce);
        let creator_cut = self.get_creator_cut(&offer_amount, &token_data);
        self.set_creator_last_withdrawal_epoch_if_empty(&token_data.creator);
        self.increase_creator_royalties(&token_data.creator, &creator_cut);

        let platform_cut = self.get_platform_cut(&offer_amount);
        self.increase_platform_royalties(&platform_cut);

        let nft_owner_cut = &offer_amount - &platform_cut - creator_cut;
        let nft_owner_deposit = self.increate_deposit(&nft_sale_info.owner, &nft_owner_cut);

        self.send_nft(&offeror, &token_id, nonce);
        self.nft_sale_info(&nft_id).clear();
        self.offer(&offeror, &nft_id, list_timestamp).clear();

        let timestamp = self.blockchain().get_block_timestamp();
        let tx_hash = self.blockchain().get_tx_hash();
        self.accept_offer_event(
            caller.clone(),
            token_id,
            nonce,
            offeror.clone(),
            offer_amount,
            timestamp,
            tx_hash.clone(),
        );
        self.deposit_update_event(caller, nft_owner_deposit, timestamp, tx_hash.clone());
        self.deposit_update_event(offeror, offeror_deposit, timestamp, tx_hash);
        Ok(())
    }
}
