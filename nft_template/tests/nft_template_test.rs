use elrond_wasm_debug::*;
use nft_template::NftTemplate;

#[test]
fn non_zero_u64_to_string_test() {
    let context = TxContext::dummy();
    let sc = nft_template::contract_obj(context.clone());

    let num = 0u64;
    let str = sc.u64_to_string(num);
    println!("{}", std::str::from_utf8(&str.as_slice()).unwrap());
}
