#Fill this with the good values
MY_WALLET_PEM="/home/elrond/Wallets/mainnet/mainnet_thingy.pem"
PROXY="https://gateway.elrond.com"
CHAIN_ID="1"
MY_ADDRESS="erd126v5rfxenay3ma8s7qddugae24ull6mmxsl8lh5pkh5ffluqcfqseu5kr8"
MY_ADDRESS_HEX="0x569941a4d99f491df4f0f01ade23b95579ffeb7b343e7fde81b5e894ff80c241"

MY_TOKEN_NAME="0x54454d50"
MY_TOKEN_TICKER="0x54454d50"

WASM="../output/nft_template.wasm"
MY_TOKEN_ID="0x54454d502d616263646566" #Fill this after issue
ROYALTIES=0x02EE #7.5%
MY_TOKEN_NAME_BASE="0x54454d504c415445"
MY_TOKEN_IMAGE_BASE_URI="0x54454d504c415445"
MY_TOKEN_IMAGE_EXTENSION="0x2e706e67"
MY_TOKEN_METADATA_BASE_URI="0x54454d504c415445"
PRICE=1000000000000000000 #1EGLD
MAX_SUPPLY=10000
SALE_START=0

deploy() {
    erdpy --verbose contract deploy --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=100000000 \
        --arguments ${MY_ADDRESS_HEX} ${MY_TOKEN_ID} ${ROYALTIES} ${MY_TOKEN_NAME_BASE} ${MY_TOKEN_IMAGE_BASE_URI} ${MY_TOKEN_IMAGE_EXTENSION} ${PRICE} ${MAX_SUPPLY} ${SALE_START} ${MY_TOKEN_METADATA_BASE_URI} \
        --send || return
}

CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqz9ktexrk53p6v43rhc93723vhxxj95qxcfqsmngnk9"
CONTRACT_ADDRESS_HEX="0x00000000000000000500116cbc9876a443a65623be0b1f2a2cb98d22d006c241"

upgrade() {
    erdpy --verbose contract upgrade ${CONTRACT_ADDRESS} --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=100000000 \
        --arguments ${MY_ADDRESS_HEX} ${MY_TOKEN_ID} ${ROYALTIES} ${MY_TOKEN_NAME_BASE} ${MY_TOKEN_IMAGE_BASE_URI} ${MY_TOKEN_IMAGE_EXTENSION} ${PRICE} ${MAX_SUPPLY} ${SALE_START} ${MY_TOKEN_METADATA_BASE_URI} \
        --send || return
}

setSpecialRole() {
    erdpy --verbose contract call erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u \
        --pem=${MY_WALLET_PEM} \
        --recall-nonce \
        --gas-limit 60000000 \
        --function "setSpecialRole" \
        --arguments ${MY_TOKEN_ID} ${CONTRACT_ADDRESS_HEX} 0x45534454526f6c654e4654437265617465 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

mintTokens() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
        --pem=${MY_WALLET_PEM} \
        --recall-nonce \
        --value 10000000000000000 \
        --gas-limit 60000000 \
        --function "mintTokens" \
        --arguments $1 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

requestWithdraw() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
        --pem=${MY_WALLET_PEM} \
        --recall-nonce \
        --gas-limit 120000000 \
        --function "requestWithdraw" \
        --arguments $1 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

withdraw() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
        --pem=${MY_WALLET_PEM} \
        --recall-nonce \
        --gas-limit 120000000 \
        --function "withdraw" \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}
