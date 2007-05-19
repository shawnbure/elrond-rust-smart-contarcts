#Fill this with the good values
MY_WALLET_PEM="WalletKey.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
MY_ADDRESS="erd17s2pz8qrds6ake3qwheezgy48wzf7dr5nhdpuu2h4rr4mt5rt9ussj7xzh"

MY_TOKEN_NAME="0x415045" #Example: 0x4d594e4654
MY_TOKEN_TICKER="0x415045" #Example: 0x4d594e4654

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

WASM="../output/nft_template.wasm"
MY_TOKEN_ID="0x4150452d353437356136" #Fill this after issue
ROYALTIES=0x02EE #7.5%
MY_TOKEN_BASE_URI="0x68747470733a2f2f67616c6163746963617065732e6d7970696e6174612e636c6f75642f697066732f516d63583667327858694650356a31694166585245755039457563525270754d43416e6f596156596a74724a654b"
PRICE=1000000000000000000 #1EGLD
MAX_SUPPLY=10000

deploy() {
    erdpy --verbose contract deploy --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=100000000 \
        --arguments ${MY_TOKEN_ID} ${ROYALTIES} ${MY_TOKEN_BASE_URI} ${PRICE} ${MAX_SUPPLY} \
        --send || return
}

CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgq3uvfynvpvcs8aldhuyrseuyepmp0cj7at9usgefv56"
CONTRACT_ADDRESS_HEX="0x000000000000000005008f18924d8166207efdb7e1070cf0990ec2fc4bdd5979"

upgrade() {
    erdpy --verbose contract upgrade ${CONTRACT_ADDRESS} --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=100000000 \
        --arguments ${MY_TOKEN_ID} ${ROYALTIES} ${MY_TOKEN_BASE_URI} ${PRICE} ${MAX_SUPPLY} \
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
        --value 3000000000000000000 \
        --gas-limit 60000000 \
        --function "mintTokens" \
        --arguments $1 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

