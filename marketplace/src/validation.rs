elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::config::BP;
use crate::storage::AuctionInfo;
use crate::storage::NftSaleInfo;

use super::config;
use super::storage;
use super::storage::NftId;
use super::utils;

#[elrond_wasm::module]
pub trait ValidationModule:
    storage::StorageModule + config::ConfigModule + utils::UtilsModule
{
    fn require_nft_for_sale(&self, nft_id: &NftId) -> SCResult<()> {
        require!(!self.nft_sale_info(nft_id).is_empty(), "Nft not for sale");
        Ok(())
    }

    fn require_nft_not_for_sale(&self, nft_id: &NftId) -> SCResult<()> {
        require!(self.nft_sale_info(nft_id).is_empty(), "Nft is for sale");
        Ok(())
    }

    fn require_valid_token_id(&self, token_id: &TokenIdentifier) -> SCResult<()> {
        require!(token_id.is_valid_esdt_identifier(), "Invalid token Id");
        Ok(())
    }

    fn require_valid_nonce(&self, nonce: u64) -> SCResult<()> {
        require!(nonce != 0, "Invalid nonce");
        Ok(())
    }

    fn require_valid_nft_amount(&self, amount: &Self::BigUint) -> SCResult<()> {
        require!(amount == &1, "Invalid amount");
        Ok(())
    }

    fn require_valid_price(&self, price: &Self::BigUint) -> SCResult<()> {
        require!(self.get_platform_cut(price) != 0, "Invalid price");

        let min_price = self.asset_min_price().get();
        require!(price >= &min_price, "Price too low");
        require!(price % &min_price == 0, "Price has to be multiple of min");

        require!(price <= &self.asset_max_price().get(), "Price too high");
        Ok(())
    }

    fn require_valid_royalties(&self, token_data: &EsdtTokenData<Self::BigUint>) -> SCResult<()> {
        let platform_fee = self.get_platform_fee_percent_or_default();
        require!(
            token_data.royalties <= self.get_royalties_max_fee_percent_or_default(),
            "Royalties too big"
        );
        require!(
            &token_data.royalties + &platform_fee.into() < BP,
            "Royalties too big"
        );
        Ok(())
    }

    fn require_uris_not_empty(&self, token_data: &EsdtTokenData<Self::BigUint>) -> SCResult<()> {
        require!(!token_data.uris.is_empty(), "Empty uris");
        Ok(())
    }

    fn require_owns_nft(
        &self,
        address: &Address,
        nft_sale_info: &NftSaleInfo<Self::BigUint>,
    ) -> SCResult<()> {
        require!(address == &nft_sale_info.owner, "Not owner");
        Ok(())
    }

    fn require_not_owns_nft(
        &self,
        address: &Address,
        nft_sale_info: &NftSaleInfo<Self::BigUint>,
    ) -> SCResult<()> {
        require!(address != &nft_sale_info.owner, "Is owner");
        Ok(())
    }

    fn require_same_amounts(&self, a: &Self::BigUint, b: &Self::BigUint) -> SCResult<()> {
        require!(a == b, "Amounts differ");
        Ok(())
    }

    fn require_has_amount_in_deposit(
        &self,
        address: &Address,
        amount: &Self::BigUint,
    ) -> SCResult<()> {
        require!(
            &self.egld_deposit(address).get() >= amount,
            "not enough amount in deposit"
        );
        Ok(())
    }

    fn require_offer_exists(
        &self,
        address: &Address,
        nft_id: &NftId,
        timestamp: u64,
    ) -> SCResult<()> {
        require!(
            !self.offer(address, nft_id, timestamp).is_empty(),
            "offer does not exist"
        );
        Ok(())
    }

    fn require_valid_start_time(&self, start: u64, current: u64) -> SCResult<()> {
        require!(start >= current, "start time in the past");
        Ok(())
    }

    fn require_valid_deadline(&self, deadline: u64, start: u64) -> SCResult<()> {
        require!(deadline >= start, "deadline before start");
        Ok(())
    }

    fn require_nft_not_on_auction(&self, nft_id: &NftId) -> SCResult<()> {
        require!(self.auction(nft_id).is_empty(), "Nft is on auction");
        Ok(())
    }

    fn require_nft_on_auction(&self, nft_id: &NftId) -> SCResult<()> {
        require!(!self.auction(nft_id).is_empty(), "Nft is not on auction");
        Ok(())
    }

    fn require_not_auction_owner(
        &self,
        address: &Address,
        auction_info: &AuctionInfo<Self::BigUint>,
    ) -> SCResult<()> {
        require!(address != &auction_info.owner, "Is owner");
        Ok(())
    }

    fn require_auction_owner(
        &self,
        address: &Address,
        auction_info: &AuctionInfo<Self::BigUint>,
    ) -> SCResult<()> {
        require!(address == &auction_info.owner, "Not owner");
        Ok(())
    }

    fn require_is_auction_ongoing(
        &self,
        auction_info: &AuctionInfo<Self::BigUint>,
    ) -> SCResult<()> {
        let current_time = self.blockchain().get_block_timestamp();
        require!(
            auction_info.start_time <= current_time,
            "Auction did not start"
        );
        require!(current_time <= auction_info.deadline, "Auction ended");
        Ok(())
    }

    fn require_valid_new_bid(
        &self,
        new_bid: &Self::BigUint,
        auction_info: &AuctionInfo<Self::BigUint>,
    ) -> SCResult<()> {
        require!(new_bid >= &auction_info.min_bid, "Lower than min bid");
        require!(new_bid > &auction_info.bid, "Lower than highest bid");
        Ok(())
    }

    fn require_deadline_passed(&self, auction_info: &AuctionInfo<Self::BigUint>) -> SCResult<()> {
        let current_time = self.blockchain().get_block_timestamp();
        require!(
            current_time > auction_info.deadline,
            "Auction deadline not passed"
        );
        Ok(())
    }

    fn require_auction_ended_or_not_started(
        &self,
        auction_info: &AuctionInfo<Self::BigUint>,
    ) -> SCResult<()> {
        let current_time = self.blockchain().get_block_timestamp();
        require!(
            auction_info.is_ended || current_time < auction_info.start_time,
            "Action started and did not end"
        );
        Ok(())
    }
}
