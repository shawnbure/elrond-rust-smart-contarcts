elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::storage;

pub const BP: u64 = 10_000;
pub const DEFAULT_FEE_PERCENT: u64 = 250;
pub const ROYALTIES_MAX_FEE_PERCENT: u64 = 1_000;

#[elrond_wasm::module]
pub trait ConfigModule: storage::StorageModule {
    #[only_owner]
    #[endpoint(setPlatformFeePercent)]
    fn set_platform_fee_percent(&self, fee_percent: u64) -> SCResult<()> {
        self.try_set_platform_fee_percent(fee_percent)
    }

    #[only_owner]
    #[endpoint(setRoyaltiesMaxFeePercent)]
    fn set_royalties_max_fee_percent(&self, fee_percent: u64) -> SCResult<()> {
        self.try_set_royalties_max_fee_percent(fee_percent)
    }

    #[only_owner]
    #[endpoint(setAssetPriceRange)]
    fn set_asset_price_range(
        &self,
        min_price: Self::BigUint,
        max_price: Self::BigUint,
    ) -> SCResult<()> {
        self.try_set_asset_price_range(min_price, max_price)
    }

    #[only_owner]
    #[endpoint(setCreatorWithdrawalWaitingEpochs)]
    fn set_creator_withdrawal_waiting_epochs(&self, epochs: u64) -> SCResult<()> {
        self.try_set_creator_withdrawal_waiting_epochs(epochs)
    }

    #[only_owner]
    #[endpoint(blacklistCreator)]
    fn blacklist_creator(&self, address: Address) {
        self.creator_blacklist(&address).set(&true);
    }

    #[only_owner]
    #[endpoint(removeCreatorFromBlacklist)]
    fn remove_creator_from_blacklist(&self, address: Address) {
        self.creator_blacklist(&address).set(&false);
    }

    fn try_set_platform_fee_percent(&self, fee_percent: u64) -> SCResult<()> {
        require!(fee_percent < BP, "Fee percent too high");
        require!(fee_percent != 0, "Fee percent cannot be zero");
        self.platform_fee_percent().set(&fee_percent);
        Ok(())
    }

    fn try_set_asset_price_range(
        &self,
        min_price: Self::BigUint,
        max_price: Self::BigUint,
    ) -> SCResult<()> {
        require!(min_price != 0, "Min price cannot be zero");
        require!(max_price >= min_price, "Max cannot be lower than min");
        self.asset_min_price().set(&min_price);
        self.asset_max_price().set(&max_price);
        Ok(())
    }

    fn try_set_royalties_max_fee_percent(&self, fee_percent: u64) -> SCResult<()> {
        require!(
            fee_percent <= ROYALTIES_MAX_FEE_PERCENT,
            "Royalties fee too high"
        );
        self.royalties_max_fee_percent().set(&fee_percent);
        Ok(())
    }

    fn try_set_creator_withdrawal_waiting_epochs(&self, epochs: u64) -> SCResult<()> {
        self.creator_withdrawal_waiting_epochs().set(&epochs);
        Ok(())
    }

    fn try_set_admin_pub_key(&self, pub_key: BoxedBytes) -> SCResult<()> {
        self.admin_pub().set(&pub_key);
        Ok(())
    }
}
