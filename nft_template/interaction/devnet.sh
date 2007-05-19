#Fill this with the good values
MY_WALLET_PEM="WalletKey.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
MY_ADDRESS="erd17s2pz8qrds6ake3qwheezgy48wzf7dr5nhdpuu2h4rr4mt5rt9ussj7xzh"

MY_TOKEN_NAME="0x57414d454e" #Example: 0x4d594e4654
MY_TOKEN_TICKER="0x57414d454e" #Example: 0x4d594e4654

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
MY_TOKEN_ID="0x57414d454e2d303833623530" #Fill this after issue
ROYALTIES=0x02EE #7.5%
MY_TOKEN_BASE_URI="0x68747470733a2f2f776f772d70726f642d6e6674726962652e73332e65752d776573742d322e616d617a6f6e6177732e636f6d2f74"
PRICE=1000000000000000000 #1EGLD

deploy() {
    erdpy --verbose contract deploy --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=100000000 \
        --arguments ${MY_TOKEN_ID} ${ROYALTIES} ${MY_TOKEN_BASE_URI} ${PRICE} \
        --send || return
}

CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgq62cm9582mf7435qdts9hunk84hnr0ekft9us7flg4l"
CONTRACT_ADDRESS_HEX="0x00000000000000000500d2b1b2d0eada7d58d00d5c0b7e4ec7ade637e6c95979"

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

mintNft() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
        --pem=${MY_WALLET_PEM} \
        --recall-nonce \
        --value 1000000000000000000 \
        --gas-limit 60000000 \
        --function "mintNft" \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

