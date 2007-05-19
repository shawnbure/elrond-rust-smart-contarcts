#Fill this with the good values
MY_WALLET_PEM="WalletKey.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
MY_ADDRESS="erd17s2pz8qrds6ake3qwheezgy48wzf7dr5nhdpuu2h4rr4mt5rt9ussj7xzh"

MY_TOKEN_NAME="YOUR_TOKEN_NAME_HERE_IN_HEX" #Example: 0x4d594e4654
MY_TOKEN_TICKER="YOUR_TOKEN_TICKER_HERE_IN_HEX" #Example: 0x4d594e4654

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
MY_TOKEN_ID="0x414243" #Fill this after issue
ROYALTIES=0x02EE #7.5%
MY_TOKEN_BASE_URI="0xDEADBEEF"
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

CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgq6ywxegjzqcpavljdtfmnt05h63a9d0zvt9usr9vez2"
CONTRACT_ADDRESS_HEX="0x00000000000000000500d11c6ca2420603d67e4d5a7735be97d47a56bc4c5979"

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

