#Fill this with the good values
MY_WALLET_PEM="../../dev-wallet-owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"

MARKETPLACE_ADDRESS="erd1qqqqqqqqqqqqqpgq24kgq27esq064w3rqqxhlusld4hw9kmzy4wss4zf2s"             #this is from Marketplace contract "CONTRACT_ADDRESS" devnet.sh
MARKETPLACE_ADDRESS_HEX="0x00000000000000000500556c802bd9801faaba23000d7ff21f6d6ee2db62255d"     #erdpy wallet bech32 --decode <MY_ADDRESS> to get this value


#SET THIS AFTER DEPLOYMENT
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqhxqvt5fxrgj4awh955a0r8mlkpwhfm24y4ws7j90gc"
CONTRACT_ADDRESS_HEX="0x00000000000000000500b980c5d1261a255ebae5a53af19f7fb05d74ed55255d"


#  erdpy wallet bech32 --decode erd1qqqqqqqqqqqqqpgqw5hck8z6qvsmmpzfvm7d0v8y35hnknxdy4ws5yth92
# 0x00000000000000000500752f8b1c5a0321bd844966fcd7b0e48d2f3b4ccd255d

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
MY_TOKEN_ID="0x434855422d626165633062" #Fill this after issue
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
        --gas-limit=600000000 \
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


enableWhiteList(){
    erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqfudlc5h5t0ulcr9vg2yepwaz9nm3ngkgy4wsu38rtm \
        --pem=wufq.pem \
        --recall-nonce \
        --gas-limit 4341625 \
        --function "updateBuyerWhitelistCheck" \
        --arguments 1 \
        --proxy="https://devnet-gateway.elrond.com" \
        --chain="D" \
        --send
}

# Run first
# deploy  


#-------- SHELL EXECUTED FUNCTIONS --------------

# RUN THIS INITIAL TO GET TOKEN ID 
#issueNft

# DEPLOY AFTER YOU ISSUE_NFT
upgrade

#------------------------------------------------