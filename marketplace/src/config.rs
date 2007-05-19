elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::storage;

pub const BP: u64 = 10_000;
pub const DEFAULT_FEE_PERCENT: u64 = 250;

#[elrond_wasm::module]
pub trait ConfigModule: storage::StorageModule {
    #[only_owner]
    #[endpoint(withdraw)]
    fn withdraw(&self) {
        let caller = &self.blockchain().get_caller();
        let sc_address = &self.blockchain().get_sc_address();
        let balance = &self.blockchain().get_balance(sc_address);
        self.send().direct_egld(caller, balance, &[]);
    }

    #[only_owner]
    #[endpoint(setPlatformFeePercent)]
    fn set_platform_fee_percent(&self, fee_percent: u64) -> SCResult<()> {
        self.try_set_platform_fee_percent(fee_percent)
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
        require!(max_price != 0, "Max price cannot be zero");
        require!(max_price >= min_price, "Max cannot be lower than min");
        self.asset_min_price().set(&min_price);
        self.asset_max_price().set(&max_price);
        Ok(())
    }
}
