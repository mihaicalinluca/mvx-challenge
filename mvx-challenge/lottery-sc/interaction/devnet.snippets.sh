DEVWALLET="../../devWallet.pem" #wallet path
ADDRESS=$(mxpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-devnet)
WASM_PATH="/mnt/d/work/smart-contracts/mvx-challenge/lottery-sc/output/lottery-sc.wasm" 
PROXY="https://devnet-gateway.multiversx.com" #change with gateway.multiversx.com for main net
DEV_CHAIN_ID="D" #change with 1 for main net
ADDR="erd1qqqqqqqqqqqqqpgqm00fawx8ce3pftwykrvxucwm56rehcnp0ytss75xll"

deploy() {
     mxpy --verbose contract deploy \
    --bytecode=${WASM_PATH} \
    --recall-nonce \
    --pem=${DEVWALLET} \
    --gas-limit=500000000 \
    --metadata-payable \
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
    --proxy=${PROXY} \
    --chain=${DEV_CHAIN_ID} \
    --send || return

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}