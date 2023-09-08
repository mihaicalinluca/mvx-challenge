use multi_token_contract::*;
use multiversx_sc::storage::mappers::SingleValue;
use multiversx_sc_scenario::{api::StaticApi, num_bigint::BigUint, scenario_model::*, *};

const lib_PATH_EXPR: &str = "file:output/multi-token-contract.wasm";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/lib");

    blockchain.register_contract("file:output/multi-token-contract.wasm", multi_token_contract::ContractBuilder);
    blockchain
}

#[test]
fn lib_blackbox_with_values() {
    let mut world = world();
    let owner_address = "address:owner";
    let mut lib_contract = ContractInfo::<lib::Proxy<StaticApi>>::new("sc:lib");
    let lib_code = world.code_expression(lib_PATH_EXPR);

    world
        .start_trace()
        .set_state_step(
            SetStateStep::new()
                .put_account(owner_address, Account::new().nonce(1))
                .new_address(owner_address, 1, "sc:lib"),
        )
        .sc_deploy_use_result(
            ScDeployStep::new()
                .from(owner_address)
                .code(lib_code)
                .call(lib_contract.init(5u32)),
            |new_address, _: TypedResponse<()>| {
                assert_eq!(new_address, lib_contract.to_address());
            },
        )
        .sc_query(
            ScQueryStep::new()
                .to(&lib_contract)
                .call(lib_contract.sum())
                .expect_value(SingleValue::from(BigUint::from(5u32))),
        )
        .sc_call(
            ScCallStep::new()
                .from(owner_address)
                .to(&lib_contract)
                .call(lib_contract.add(3u32)),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account(owner_address, CheckAccount::new())
                .put_account(
                    &lib_contract,
                    CheckAccount::new().check_storage("str:sum", "8"),
                ),
        )
        .write_scenario_trace("trace1.scen.json");
}
