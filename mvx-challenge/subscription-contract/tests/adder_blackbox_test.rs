use multiversx_sc_scenario::{scenario_model::*, *};

const lib_PATH_EXPR: &str = "file:output/subscription-contract.wasm";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/lib");

    blockchain.register_contract(lib_PATH_EXPR, subscription_contract::ContractBuilder);
    blockchain
}

#[test]
fn lib_blackbox_raw() {
    let mut world = world();
    let lib_code = world.code_expression(lib_PATH_EXPR);

    world
        .set_state_step(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(1))
                .new_address("address:owner", 1, "sc:lib"),
        )
        .sc_deploy(
            ScDeployStep::new()
                .from("address:owner")
                .code(lib_code)
                .argument("5")
                .expect(TxExpect::ok().no_result()),
        )
        .sc_query(
            ScQueryStep::new()
                .to("sc:lib")
                .function("getSum")
                .expect(TxExpect::ok().result("5")),
        )
        .sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:lib")
                .function("add")
                .argument("3")
                .expect(TxExpect::ok().no_result()),
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
