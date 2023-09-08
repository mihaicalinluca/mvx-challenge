use subscription_contract::*;
use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::api::SingleTxApi;

#[test]
fn lib_unit_test() {
    let lib = subscription_contract::contract_obj::<SingleTxApi>();

    lib.init(BigUint::from(5u32));
    assert_eq!(BigUint::from(5u32), lib.sum().get());

    lib.add(BigUint::from(7u32));
    assert_eq!(BigUint::from(12u32), lib.sum().get());

    lib.add(BigUint::from(1u32));
    assert_eq!(BigUint::from(13u32), lib.sum().get());
}
