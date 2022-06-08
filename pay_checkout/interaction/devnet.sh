MY_WALLET_PEM="../../dev-wallet-owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
WASM="../output/pay_checkout.wasm"
VERSION="0.0.1"
VERSION_HEX=0x302E302E31
CHECKOUT_AMOUNT=200000000000000000
CHECKOUT_ID=5

CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqkl74d07529024m0qeh02ua3t928g3vyyk8ks65njt2"             #this is from Marketplace contract "CONTRACT_ADDRESS" devnet.sh
CONTRACT_ADDRESS_HEX="0x00000000000000000500b7fd56bfd4515eaaede0cddeae762b2a8e88b084b1ed"     #erdpy wallet bech32 --decode <MY_ADDRESS> to get this value

MARKETPLACE_ADDRESS="erd1qqqqqqqqqqqqqpgqznqdfhrghvmhuzjlg3emess422hpmazrk8ksuvarcd"           #after deploying, the the contract address
MARKETPLACE_ADDRESS_HEX="0x0000000000000000050014c0d4dc68bb377e0a5f4473bcc21552ae1df443b1ed"   #erdpy wallet bech32 --decode <CONTRACT_ADDRESS> to get this value


deploy() {
    erdpy --verbose contract deploy --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=100000000 \
        --send || return
}

upgrade() {
    erdpy --verbose contract upgrade ${CONTRACT_ADDRESS} --recall-nonce \
         --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=100000000 \
        --send || return
}

payCheckout() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
        --pem=${MY_WALLET_PEM} \
        --recall-nonce \
        --gas-limit 120000000 \
        --function payCheckout \
        --arguments ${MARKETPLACE_ADDRESS} ${CHECKOUT_AMOUNT} ${CHECKOUT_ID} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

checkDepositStatus() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function getStatus \
        --arguments ${CHECKOUT_ID}
}

#-------- SHELL EXECUTED FUNCTIONS --------------

checkDepositStatus

#------------------------------------------------
