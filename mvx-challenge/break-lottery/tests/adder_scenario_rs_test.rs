use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/lib");

    blockchain.register_contract("file:output/break-lottery.wasm", break_lottery::ContractBuilder);
    blockchain
}

#[test]
fn lib_rs() {
    world().run("scenarios/lib.scen.json");
}

#[test]
fn interactor_trace_rs() {
    world().run("scenarios/interactor_trace.scen.json");
}
