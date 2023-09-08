use subscription_contract::*;
use multiversx_sc_scenario::{scenario_model::*, *};

const lib_PATH_EXPR: &str = "file:output/subscription-contract.wasm";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/lib");

    blockchain.register_contract("file:output/subscription-contract.wasm", subscription_contract::ContractBuilder);
    blockchain
}

#[test]
fn lib_whitebox() {
    let mut world = world();
    let lib_whitebox = WhiteboxContract::new("sc:lib", subscription_contract::contract_obj);
    let lib_code = world.code_expression(lib_PATH_EXPR);

    world
        .set_state_step(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(1))
                .new_address("address:owner", 1, "sc:lib"),
        )
        .whitebox_deploy(
            &lib_whitebox,
            ScDeployStep::new().from("address:owner").code(lib_code),
            |sc| {
                sc.init(5u32.into());
            },
        )
        .whitebox_query(&lib_whitebox, |sc| {
            let sum_value = sc.sum();
            assert_eq!(sum_value.get(), 5u32);
        })
        .whitebox_call(
            &lib_whitebox,
            ScCallStep::new().from("address:owner"),
            |sc| sc.add(3u32.into()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account("address:owner", CheckAccount::new())
                .put_account(
                    "sc:lib",
                    CheckAccount::new().check_storage("str:sum", "8"),
                ),
        );
}
