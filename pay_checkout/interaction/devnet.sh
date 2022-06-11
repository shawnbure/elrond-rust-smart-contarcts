MY_WALLET_PEM="../../dev-wallet-owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
WASM="../output/pay_checkout.wasm"
VERSION="0.0.1"
VERSION_HEX=0x302E302E31
CHECKOUT_AMOUNT=100000000000000000
CHECKOUT_ID=5

CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqxgzj7j3u3jxgh7mwgt9lztjlny3njvr3y4wszzjj7w"             #this is from Marketplace contract "CONTRACT_ADDRESS" devnet.sh
CONTRACT_ADDRESS_HEX="0x0000000000000000050032052f4a3c8c8c8bfb6e42cbf12e5f9923393071255d"     #erdpy wallet bech32 --decode <MY_ADDRESS> to get this value

MARKETPLACE_ADDRESS="erd1qqqqqqqqqqqqqpgqjq5wlj36spzvf9ppq2ae242wv9a78avcy4ws8ktslw"           #after deploying, the the contract address
MARKETPLACE_ADDRESS_HEX="0x000000000000000005009028efca3a8044c4942102bb95554e617be3f598255d"   #erdpy wallet bech32 --decode <CONTRACT_ADDRESS> to get this value


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

upgrade

#------------------------------------------------
