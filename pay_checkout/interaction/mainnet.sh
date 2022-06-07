MY_WALLET_PEM="../../dev-wallet-owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
WASM="../output/pay_checkout.wasm"
VERSION="0.0.1"
VERSION_HEX=0x302E302E31

CONTRACT_ADDRESS = "erd1qqqqqqqqqqqqqpgq0rmc7xxd3aku7ll428zlcla6nfyrxqcvy4ws242807"             #this is from Marketplace contract "CONTRACT_ADDRESS" devnet.sh
CONTRACT_ADDRESS_HEX = "0x0000000000000000050078f78f18cd8f6dcf7ff551c5fc7fba9a4833030c255d"     #erdpy wallet bech32 --decode <MY_ADDRESS> to get this value

MARKETPLACE_ADDRESS="erd1qqqqqqqqqqqqqpgqjq5wlj36spzvf9ppq2ae242wv9a78avcy4ws8ktslw"             
MARKETPLACE_ADDRESS_HEX="0x000000000000000005009028efca3a8044c4942102bb95554e617be3f598255d"     #erdpy wallet bech32 --decode <MY_ADDRESS> to get this value


deploy() {
    erdpy --verbose contract deploy --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=200000000 \
        --arguments ${VERSION_HEX} \
        --send || return
}

upgrade() {
    erdpy --verbose contract upgrade ${CONTRACT_ADDRESS} --recall-nonce \
         --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=100000000 \
        --arguments ${VERSION_HEX} \
        --send || return
}

#-------- SHELL EXECUTED FUNCTIONS --------------

deploy

#------------------------------------------------