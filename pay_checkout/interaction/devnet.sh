MY_WALLET_PEM="../../dev-wallet-owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
WASM="../output/pay_checkout.wasm"
VERSION="0.0.1"
VERSION_HEX=0x302E302E31
CHECKOUT_AMOUNT=100000000000000000
CHECKOUT_ID=5

CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqxqkm7yn3k9ymv7t0flqkfplwwgmt0n6tk8ksallkyj"             #this is from Marketplace contract "CONTRACT_ADDRESS" devnet.sh
CONTRACT_ADDRESS_HEX="0x00000000000000000500302dbf1271b149b6796f4fc16487ee7236b7cf4bb1ed"     #erdpy wallet bech32 --decode <MY_ADDRESS> to get this value

MARKETPLACE_ADDRESS="erd1qqqqqqqqqqqqqpgqasnr44wwyyj8cg7s9ryd5sdhra3tf09zk8kstyek5n"           #after deploying, the the contract address
MARKETPLACE_ADDRESS_HEX="0x00000000000000000500ec263ad5ce21247c23d028c8da41b71f62b4bca2b1ed"   #erdpy wallet bech32 --decode <CONTRACT_ADDRESS> to get this value


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

payCheckout

#------------------------------------------------
