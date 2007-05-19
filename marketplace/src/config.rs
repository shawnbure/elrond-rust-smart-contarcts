elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::storage;

pub const BP: u64 = 10_000;
pub const DEFAULT_FEE_PERCENT: u64 = 250;
pub const MAX_COLLECTION_NAME_LEN: usize = 30;
pub const MAX_DESCRIPTION_LEN: usize = 500;

#[elrond_wasm::module]
pub trait ConfigModule: storage::StorageModule {
    #[only_owner]
    #[endpoint(setPlatformFeePercent)]
    fn set_platform_fee_percent(&self, fee_percent: u64) -> SCResult<()> {
        self.try_set_platform_fee_percent(fee_percent)
    }

    #[only_owner]
    #[endpoint(setCollectionRegisterPrice)]
    fn set_collection_register_price(&self, price: Self::BigUint) {
        self.collection_register_price().set(&price);
    }

    #[only_owner]
    #[endpoint(withdraw)]
    fn withdraw(&self) {
        let caller = &self.blockchain().get_caller();
        let sc_address = &self.blockchain().get_sc_address();
        let balance = &self.blockchain().get_balance(sc_address);
        self.send().direct_egld(caller, balance, &[]);
    }

    fn try_set_platform_fee_percent(&self, fee_percent: u64) -> SCResult<()> {
        require!(fee_percent < BP, "Fee percent too high");
        require!(fee_percent != 0, "Fee percent cannot be zero");
        self.platform_fee_percent().set(&fee_percent);
        Ok(())
    }
}
