MY_WALLET_PEM="../../dev-wallet-owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
WASM="../output/pay_checkout.wasm"
VERSION="0.0.1"
VERSION_HEX=0x302E302E31

CONTRACT_ADDRESS = "erd1qqqqqqqqqqqqqpgqg3h7rd5r5057ug209hlczez5hmq3upt9k8ks0arkq7"             #this is from Marketplace contract "CONTRACT_ADDRESS" devnet.sh
CONTRACT_ADDRESS_HEX = "0x00000000000000000500446fe1b683a3e9ee214f2dff816454bec11e0565b1ed"     #erdpy wallet bech32 --decode <MY_ADDRESS> to get this value

MARKETPLACE_ADDRESS="erd1qqqqqqqqqqqqqpgqjq5wlj36spzvf9ppq2ae242wv9a78avcy4ws8ktslw"             
MARKETPLACE_ADDRESS_HEX="0x000000000000000005009028efca3a8044c4942102bb95554e617be3f598255d"     #erdpy wallet bech32 --decode <MY_ADDRESS> to get this value


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
    erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqg3h7rd5r5057ug209hlczez5hmq3upt9k8ks0arkq7 \
        --pem=${MY_WALLET_PEM} \
        --recall-nonce \
        --gas-limit 120000000 \
        --function "payCheckout" \
        --arguments ${MARKETPLACE_ADDRESS} $1 $1 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

#-------- SHELL EXECUTED FUNCTIONS --------------

payCheckout

#------------------------------------------------
