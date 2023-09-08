DEVWALLET="../../devWallet.pem" #wallet path
ADDRESS=$(mxpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-devnet)
WASM_PATH="/mnt/d/work/smart-contracts/mvx-challenge/break-lottery/output/break-lottery.wasm" 
PROXY="https://devnet-gateway.multiversx.com" #change with gateway.multiversx.com for main net
DEV_CHAIN_ID="D" #change with 1 for main net
ADDR="erd1qqqqqqqqqqqqqpgqagltns9qc97av5nw7hw0xz2veuds8s4n0yts4vgjf7"
CALEE_SC_ADDR_HEX="0x00000000000000000500dbde9eb8c7c66214adc4b0d86e61dba6879be2617917"

deploy() {
     mxpy --verbose contract deploy \
    --bytecode=${WASM_PATH} \
    --recall-nonce \
    --pem=${DEVWALLET} \
    --gas-limit=500000000 \
    --metadata-payable \
    --arguments ${CALEE_SC_ADDR_HEX}\
    --proxy=${PROXY} \
    --chain=${DEV_CHAIN_ID} \
    --send || return

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
     mxpy --verbose contract upgrade ${ADDR} \
    --bytecode=${WASM_PATH} \
    --recall-nonce \
    --pem=${DEVWALLET} \
    --gas-limit=500000000 \
    --metadata-payable \
    --arguments ${CALEE_SC_ADDR_HEX}\
    --proxy=${PROXY} \
    --chain=${DEV_CHAIN_ID} \
    --send || return

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}