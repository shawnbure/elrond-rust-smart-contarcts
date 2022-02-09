#Fill this with the good values
MY_WALLET_PEM="../../dev-wallet-owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"

MARKETPLACE_ADDRESS="erd1qqqqqqqqqqqqqpgquxa3w45zcxrl69hdk9l4zvk8jj5mnsu0y4wsem7pnc"             #this is from Marketplace contract "CONTRACT_ADDRESS" devnet.sh
MARKETPLACE_ADDRESS_HEX="0x00000000000000000500e1bb175682c187fd16edb17f5132c794a9b9c38f255d"     #erdpy wallet bech32 --decode <MY_ADDRESS> to get this value


#SET THIS AFTER DEPLOYMENT
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqd9tv8f3k7ea5y9wsnalhs9df5u3k7300y4wsd89yta"
CONTRACT_ADDRESS_HEX="0x000000000000000005006956c3a636f67b4215d09f7f7815a9a7236f45ef255d"


#  erdpy wallet bech32 --decode erd12ngznv48rnlvgjtuc4h7ev5e2dyy0thzgewd95edne2urhyxrwtq09ara3
# 0x54d029b2a71cfec4497cc56fecb299534847aee2465cd2d32d9e55c1dc861b96

MY_TOKEN_NAME="0x43485542"          #'CHUB'
MY_TOKEN_TICKER="0x43485542"        #'CHUB'

# 434855422d613166636239
# CHUB-a1fcb9


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

WASM="../output/nft_template.wasm"   #
MY_TOKEN_ID="0x434855422d613166636239" #Fill this after issue
ROYALTIES=0x02EE #7.5%
MY_TOKEN_NAME_BASE="0x4368756262696672656e73"   #Chubbifrens
MY_TOKEN_IMAGE_BASE_URI="0x68747470733a2f2f7777772e63687562626976657273652e636f6d2f6672656e"    #https://www.chubbiverse.com/fren
MY_TOKEN_IMAGE_EXTENSION="0x2e706e67"           #.png
MY_TOKEN_METADATA_BASE_URI="0x68747470733a2f2f7777772e63687562626976657273652e636f6d2f6170692f6d6574612f31" #https://www.chubbiverse.com/api/meta/1
PRICE=1000000000000000000 #1EGLD  (18 places)
MAX_SUPPLY=10000
SALE_START=0


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


# Run first
# issueNft  


#-------- SHELL EXECUTED FUNCTIONS --------------

# RUN THIS INITIAL TO GET TOKEN ID 
issueNft

# DEPLOY AFTER YOU ISSUE_NFT
deploy

#------------------------------------------------