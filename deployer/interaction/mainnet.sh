MY_WALLET_PEM="../../../admin.pem"
PROXY="https://gateway.elrond.com"
CHAIN_ID="1"
WASM="../output/deployer.wasm"
VERSION="0.0.1"
VERSION_HEX=0x302E302E31

MY_ADDRESS="erd126"
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgq9vpuwyf6q7d0qg2guq2zn0llumvtmuspydjsj5ul79"
CONTRACT_ADDRESS_HEX="0x000000000000000005002b03c7113a079af02148e01429bfffe6d8bdf2012365"

#NFT TEMPLATE ADDRESS (devnet_chub.sh)
TEMPLATE_CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqayduf3tr40pja8gwd9huendczgn287rnydjs7rr06j"
TEMPLATE_CONTRACT_ADDRESS_HEX="0x00000000000000000500e91bc4c563abc32e9d0e696fcccdb81226a3f8732365"

MARKETPLACE_ADMIN_ADDRESS="erd1uww6wsvmxq9k88n00rpz3txyshuj2elvl0wk5y74yx3la359ydjscw6ew8"
MARKETPLACE_ADMIN_ADDRESS_HEX="0xe39da7419b300b639e6f78c228acc485f92567ecfbdd6a13d521a3fec6852365"

deploy() {
    erdpy --verbose contract deploy --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=50000000 \
        --arguments ${TEMPLATE_CONTRACT_ADDRESS_HEX} ${MARKETPLACE_ADMIN_ADDRESS_HEX} ${VERSION_HEX} \
        --send || return
}

upgrade() {
    erdpy --verbose contract upgrade ${CONTRACT_ADDRESS} --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=50000000 \
        --arguments ${TEMPLATE_CONTRACT_ADDRESS_HEX} ${MARKETPLACE_ADMIN_ADDRESS_HEX} ${VERSION_HEX} \
        --send || return
}


MY_TOKEN_NAME="0x4d4f4e4b"
MY_TOKEN_TICKER="0x4d4f4e4b"

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

MY_TOKEN_ID="0x4d4f4e4b2d623032643339"
ROYALTIES=750
TOKEN_NAME_BASE="0x4d6f6e6b"
MY_TOKEN_IMAGE_BASE_URI="0x68747470733a2f2f676174657761792e70696e6174612e636c6f75642f697066732f516d664e635535374a67383858723338503678386a764c3767386e77694b323166667439697a6d6d766a52554d57"
MY_TOKEN_IMAGE_EXTENSION="0x2e706e67"
MY_TOKEN_METADATA_BASE_URI="0x68747470733a2f2f697066732e696f2f697066732f516d536e5134434b666b6d5578396273387954796d6d55316e44426b626f43574356324c4e333355644452325854"
PRICE=1000000000000000000 #1EGLD
MAX_SUPPLY=10000
SALE_START=0

deployNFTTemplateContract() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function deployNFTTemplateContract \
        --arguments ${MY_TOKEN_ID} \
            ${ROYALTIES} \
            ${TOKEN_NAME_BASE} \
            ${MY_TOKEN_IMAGE_BASE_URI} \
            ${MY_TOKEN_IMAGE_EXTENSION} \
            ${PRICE} \
            ${MAX_SUPPLY} \
            ${SALE_START} \
            ${MY_TOKEN_METADATA_BASE_URI} \
        --gas-limit=20000000 \
        --send || return
}

DEPLOIED_CONTRACT="erd1qqqqqqqqqqqqqpgq7ehkfjuvxf85zl5le6ujxn9va6n35avnt9usehfmpk"
DEPLOIED_CONTRACT_HEX="0x00000000000000000500f66f64cb8c324f417e9fceb9234caceea71a75935979"
changeOwner() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_WALLET_PEM} \
        --value=0 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function changeOwner \
        --arguments ${DEPLOIED_CONTRACT_HEX} \
        --gas-limit=100000000 \
        --send || return
}

#Fucked this up. Won't mint but works.
setSpecialRole() {
    erdpy --verbose contract call erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u \
        --pem=${MY_WALLET_PEM} \
        --recall-nonce \
        --gas-limit 60000000 \
        --function "setSpecialRole" \
        --arguments ${MY_TOKEN_ID} ${DEPLOIED_CONTRACT_HEX} 0x45534454526f6c654e4654437265617465 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

mintTokens() {
    erdpy --verbose contract call ${DEPLOIED_CONTRACT} \
        --pem=${MY_WALLET_PEM} \
        --recall-nonce \
        --value 1000000000000000000 \
        --gas-limit 60000000 \
        --function "mintTokens" \
        --arguments $1 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

requestWithdraw() {
    erdpy --verbose contract call ${DEPLOIED_CONTRACT} \
        --pem=${MY_WALLET_PEM} \
        --recall-nonce \
        --gas-limit 120000000 \
        --function "requestWithdraw" \
        --arguments $1 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

withdraw() {
    erdpy --verbose contract call ${DEPLOIED_CONTRACT} \
        --pem=${MY_WALLET_PEM} \
        --recall-nonce \
        --gas-limit 120000000 \
        --function "withdraw" \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

upgrade