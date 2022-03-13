MY_WALLET_PEM="../../dev-wallet-owner.pem"
MY_OTHER_WALLET_PEM="../../dev-extra-wallet-owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
WASM="../output/deployer.wasm"

#MY_ADDRESS="erd17s2pz8qrds6ake3qwheezgy48wzf7dr5nhdpuu2h4rr4mt5rt9ussj7xzh"

#deployer contract address
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqs80ft4tuv6w2ckhnvncsatga48qjf0zfy4wsky8qrn"
CONTRACT_ADDRESS_HEX="0x0000000000000000050081de95d57c669cac5af364f10ead1da9c124bc49255d"


#NFT TEMPLATE ADDRESS (devnet_chub.sh)
TEMPLATE_CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqhxqvt5fxrgj4awh955a0r8mlkpwhfm24y4ws7j90gc"
TEMPLATE_CONTRACT_ADDRESS_HEX="0x00000000000000000500b980c5d1261a255ebae5a53af19f7fb05d74ed55255d"

#address of the MARKETPLACE SC
MARKETPLACE_ADMIN_ADDRESS="erd1qqqqqqqqqqqqqpgq24kgq27esq064w3rqqxhlusld4hw9kmzy4wss4zf2s"
MARKETPLACE_ADMIN_ADDRESS_HEX="0x00000000000000000500556c802bd9801faaba23000d7ff21f6d6ee2db62255d"


deploy() {
    erdpy --verbose contract deploy --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=50000000 \
        --arguments ${TEMPLATE_CONTRACT_ADDRESS_HEX} ${MARKETPLACE_ADMIN_ADDRESS_HEX} \
        --send || return
}

MY_TOKEN_NAME="0x43485542"
MY_TOKEN_TICKER="0x43485542"

# 434855422d306637626631
# CHUB-0f7bf1


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
            
MY_TOKEN_ID="0x434855422d626165633062" #"0x4d4f4e4b2d623032643339"
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


#-------- SHELL EXECUTED FUNCTIONS --------------

deploy

#------------------------------------------------