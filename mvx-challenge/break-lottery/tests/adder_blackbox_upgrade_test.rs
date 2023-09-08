use multiversx_sc_scenario::{scenario_model::*, *};

const lib_PATH_EXPR: &str = "file:output/break-lottery.wasm";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/lib");

    blockchain.register_contract("file:output/break-lottery.wasm", break_lottery::ContractBuilder);
    blockchain
}

#[test]
fn lib_blackbox_upgrade() {
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
                .code(&lib_code)
                .argument("5")
                .gas_limit("5,000,000")
                .expect(TxExpect::ok().no_result()),
        )
        .sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:lib")
                .function("upgradeContract")
                .argument(&lib_code)
                .argument("0x0502") // codeMetadata
                .argument("8") // contract argument
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
