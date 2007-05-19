#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod config;
pub mod events;
pub mod global_op;
pub mod storage;
pub mod utils;
pub mod validation;
pub mod views;

use storage::{Collection, NftId, NftSaleInfo};

const NFT_AMOUNT: u64 = 1;

#[elrond_wasm::contract]
pub trait MarketplaceContract:
    events::EventsModule
    + storage::StorageModule
    + validation::ValidationModule
    + views::ViewsModule
    + config::ConfigModule
    + utils::UtilsModule
    + global_op::GlobalOperationModule
{
    #[init]
    fn init(
        &self,
        platform_fee_percent: u64,
        collection_register_price: Self::BigUint,
    ) -> SCResult<()> {
        self.collection_register_price()
            .set(&collection_register_price);
        self.try_set_platform_fee_percent(platform_fee_percent)
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

        let uri = token_data.uris.last().unwrap().clone();
        let collection = self.get_collection_name_or_default(&token_id);

        let caller = self.blockchain().get_caller();
        let timestamp = self.blockchain().get_block_timestamp();
        let fee_percent = self.get_platform_fee_percent_or_default();
        let nft_sale_info = NftSaleInfo::new(
            caller.clone(),
            uri.clone(),
            collection.clone(),
            price.clone(),
            fee_percent,
            timestamp,
        );

        self.nft_sale_info(&nft_id).set(&nft_sale_info);
        let tx_hash = self.blockchain().get_tx_hash();
        self.put_nft_for_sale_event(
            caller, token_id, nonce, uri, collection, price, timestamp, tx_hash,
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
        self.require_good_payment(&payment, &nft_sale_info)?;

        let egld = TokenIdentifier::egld();
        let token_data = self.get_token_data(&token_id, nonce);

        let creator_cut = self.get_creator_cut(&payment, &token_data);
        self.safe_send(&token_data.creator, &egld, 0, &creator_cut);

        let platform_cut = self.get_platform_cut(&payment);
        let seller_cut = &payment - &platform_cut - creator_cut;
        self.safe_send(&nft_sale_info.owner, &egld, 0, &seller_cut);

        let nft_amount = NFT_AMOUNT.into();
        self.safe_send(&caller, &token_id, nonce, &nft_amount);

        let timestamp = self.blockchain().get_block_timestamp();
        self.nft_sale_info(&nft_id).clear();

        let tx_hash = self.blockchain().get_tx_hash();
        self.buy_nft_event(
            nft_sale_info.owner,
            caller,
            token_id,
            nonce,
            nft_sale_info.uri,
            nft_sale_info.collection,
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
        self.require_nft_for_sale(&nft_id)?;

        let caller = self.blockchain().get_caller();
        let nft_sale_info = self.nft_sale_info(&nft_id).get();
        self.require_owns_nft(&caller, &nft_sale_info)?;

        let nft_amount = NFT_AMOUNT.into();
        self.safe_send(&caller, &token_id, nonce, &nft_amount);

        let timestamp = self.blockchain().get_block_timestamp();
        self.nft_sale_info(&nft_id).clear();

        let tx_hash = self.blockchain().get_tx_hash();
        self.withdraw_nft_event(
            caller,
            token_id,
            nonce,
            nft_sale_info.uri,
            nft_sale_info.collection,
            nft_sale_info.price,
            timestamp,
            tx_hash,
        );

        Ok(())
    }

    #[payable("EGLD")]
    #[endpoint(registerCollection)]
    fn register_collection(
        &self,
        #[payment_amount] payment: Self::BigUint,
        token_id: TokenIdentifier,
        collection_name: BoxedBytes,
        description: BoxedBytes,
    ) -> SCResult<()> {
        self.require_global_op_not_ongoing()?;

        self.require_valid_token_id(&token_id)?;
        self.require_valid_collection_name(&collection_name)?;
        self.require_valid_description(&description)?;

        self.require_good_register_collection_payment(&payment)?;
        self.require_token_id_not_registered_already(&token_id)?;
        self.require_collection_name_unique(&collection_name)?;

        let collection = Collection::new(collection_name.clone(), description.clone());
        self.collections(&token_id).set(&collection);
        self.all_collection_names().insert(collection_name.clone());

        let caller = self.blockchain().get_caller();
        let timestamp = self.blockchain().get_block_timestamp();

        let tx_hash = self.blockchain().get_tx_hash();
        self.collection_register_event(
            caller,
            token_id,
            collection_name,
            description,
            timestamp,
            tx_hash,
        );
        Ok(())
    }
}
