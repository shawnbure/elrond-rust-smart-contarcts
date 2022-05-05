elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::storage;

pub const BP: u64 = 10_000;


#[elrond_wasm::module]
pub trait ConfigModule: storage::StorageModule {



}
