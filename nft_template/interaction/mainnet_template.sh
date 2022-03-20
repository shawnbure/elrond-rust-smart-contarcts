#Fill this with the good values
MY_WALLET_PEM="../../../admin.pem"
PROXY="https://gateway.elrond.com"
CHAIN_ID="1"

MARKETPLACE_ADDRESS="erd1qqqqqqqqqqqqqpgqgkpqfvzfnut2azmam4wxwd65qen8jemvydjsvvy5ch"             #this is from Marketplace contract "CONTRACT_ADDRESS" devnet.sh
MARKETPLACE_ADDRESS_HEX="0x00000000000000000500458204b0499f16ae8b7ddd5c673754066679676c2365"     #erdpy wallet bech32 --decode <MY_ADDRESS> to get this value

#SET THIS AFTER DEPLOYMENT
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqayduf3tr40pja8gwd9huendczgn287rnydjs7rr06j"
CONTRACT_ADDRESS_HEX="0x00000000000000000500e91bc4c563abc32e9d0e696fcccdb81226a3f8732365"

MY_TOKEN_NAME="0x54454d50"
MY_TOKEN_TICKER="0x54454d50"

WASM="../output/nft_template.wasm"
MY_TOKEN_ID="0x54454D502D313163373264" #Fill this after issue
ROYALTIES=0x02EE #7.5%
MY_TOKEN_NAME_BASE="0x54454d504c415445"
MY_TOKEN_IMAGE_BASE_URI="0x54454d504c415445"
MY_TOKEN_IMAGE_EXTENSION="0x2e706e67"
MY_TOKEN_METADATA_BASE_URI="0x54454d504c415445"
PRICE=1000000000000000000 #1EGLD
MAX_SUPPLY=10000
SALE_START=0

ISSUEDHASH="af5e1544ae0145d54fc67b23d611bfc83430ba79193ada4e01477318b82edfd0"

# This is how you get your token ID
issueNft() {
    erdpy --verbose contract call erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u \
        --pem=${MY_WALLET_PEM} \
        --recall-nonce \
        --value 50000000000000000 \
        --gas-limit 60000000 \
        --function "issueNonFungible" \
        --arguments ${MY_TOKEN_NAME} ${MY_TOKEN_TICKER} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}


deploy() {
    erdpy --verbose contract deploy --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=100000000 \
        --arguments ${MARKETPLACE_ADDRESS_HEX} ${MY_TOKEN_ID} ${ROYALTIES} ${MY_TOKEN_NAME_BASE} ${MY_TOKEN_IMAGE_BASE_URI} ${MY_TOKEN_IMAGE_EXTENSION} ${PRICE} ${MAX_SUPPLY} ${SALE_START} ${MY_TOKEN_METADATA_BASE_URI} \
        --send || return
}

upgrade() {
    erdpy --verbose contract upgrade ${CONTRACT_ADDRESS} --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=100000000 \
        --arguments ${MARKETPLACE_ADDRESS_HEX} ${MY_TOKEN_ID} ${ROYALTIES} ${MY_TOKEN_NAME_BASE} ${MY_TOKEN_IMAGE_BASE_URI} ${MY_TOKEN_IMAGE_EXTENSION} ${PRICE} ${MAX_SUPPLY} ${SALE_START} ${MY_TOKEN_METADATA_BASE_URI} \
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

#-------- SHELL EXECUTED FUNCTIONS --------------

# RUN THIS INITIAL TO GET TOKEN ID 
# issueNft

# DEPLOY AFTER YOU ISSUE_NFT
upgrade

#------------------------------------------------
