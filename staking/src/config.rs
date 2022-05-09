elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::storage;

pub const BP: u64 = 10_000;

pub const PAYOUT_TIME_BLOCK: u64 =  86_400u64;      //24 hours/day * 60 min/hr * 60 sec/min = 86,400 secs

pub const PAYOUT_TIME_BUFFER: u64 =  1_200u64;     // 20 mins * 60 secs = 1200 secs




#[elrond_wasm::module]
pub trait ConfigModule: storage::StorageModule {



}
