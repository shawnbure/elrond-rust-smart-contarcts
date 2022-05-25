MY_WALLET_PEM="../../dev-wallet-owner.pem"                  
MY_OTHER_WALLET_PEM="../../dev-extra-wallet-owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
WASM="../output/marketplace.wasm"
VERSION="0.0.1"
VERSION_HEX=0x302E302E31
#SETUP THIS AFTER DEPLOYMENT
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqjq5wlj36spzvf9ppq2ae242wv9a78avcy4ws8ktslw"           #after deploying, the the contract address
CONTRACT_ADDRESS_HEX="0x000000000000000005009028efca3a8044c4942102bb95554e617be3f598255d"   #erdpy wallet bech32 --decode <CONTRACT_ADDRESS> to get this value

MY_OTHER_ADDRESS="erd13rp6j2fg5wcqdztuwtt5z2n0ls8u0rplnqhyxd676mjtxd09fk7seef9ug"
MY_ADDRESS="erd17s2pz8qrds6ake3qwheezgy48wzf7dr5nhdpuu2h4rr4mt5rt9ussj7xzh"

DAO_ADDRESS="0x6b3d87c350a9fc286199e186de9e479dc9a2b58808083b7c419afbf358082319"
deploy() {
    erdpy --verbose contract deploy --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=200000000 \
        --arguments 0xfa 0x03e8 0x038D7EA4C68000 0x52B7D2DCC80CD2E4000000 0x1e ${DAO_ADDRESS} ${VERSION_HEX} \
        --send || return
}

upgrade() {
    erdpy --verbose contract upgrade ${CONTRACT_ADDRESS} --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=200000000 \
        --arguments 0xfa 0x03e8 0x038D7EA4C68000 0x52B7D2DCC80CD2E4000000 0x1e ${DAO_ADDRESS} ${VERSION_HEX} \
        --send || return
}

# $1 token id in hex
# $2 nonce
putNftForSale() {
    erdpy --verbose contract call ${MY_ADDRESS} --recall-nonce \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function ESDTNFTTransfer \
        --arguments $1 $2 0x01 ${CONTRACT_ADDRESS_HEX} 0x7075744e6674466f7253616c65 0x0DE0B6B3A7640000 \
        --gas-limit=100000000 \
        --send || return
}

# $1 token id in hex
# $2 nonce
buyNft() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_OTHER_WALLET_PEM} \
        --value=1000000000000000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function buyNft \
        --arguments $1 $2 \
        --gas-limit=100000000 \
        --send || return
}

# $1 token id in hex
# $2 nonce
withdrawNft() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function withdrawNft \
        --arguments $1 $2 \
        --gas-limit=100000000 \
        --send || return
}

# $1 amount to deposit
deposit() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --value=$1 \
        --pem=${MY_OTHER_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function deposit \
        --gas-limit=100000000 \
        --send || return
}

# $1 amount to deposit
withdraw() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_OTHER_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function withdraw \
        --gas-limit=100000000 \
        --send || return
}

# $1 token id in hex
# $2 nonce
makeOffer() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_OTHER_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --arguments $1 $2 1000000000000000000 \
        --function makeOffer \
        --gas-limit=100000000 \
        --send || return
}

acceptOffer() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --arguments $1 $2 0x88c3a92928a3b006897c72d7412a6ffc0fc78c3f982e43375ed6e4b335e54dbd 1000000000000000000 \
        --function acceptOffer \
        --gas-limit=100000000 \
        --send || return
}

cancelOffer() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_OTHER_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --arguments $1 $2 1000000000000000000 \
        --function cancelOffer \
        --gas-limit=100000000 \
        --send || return
}

startAuction() {
    erdpy --verbose contract call ${MY_ADDRESS} --recall-nonce \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function ESDTNFTTransfer \
        --arguments $1 $2 0x01 ${CONTRACT_ADDRESS_HEX} 0x737461727441756374696f6e 1000000000000000000 $3 \
        --gas-limit=100000000 \
        --send || return
}

placeBid() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_OTHER_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function placeBid \
        --arguments $1 $2 $3 \
        --gas-limit=100000000 \
        --send || return
}

endAuction() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_OTHER_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function endAuction \
        --arguments $1 $2 \
        --gas-limit=100000000 \
        --send || return
}

getRemainingEpochsUntilClaim() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function getRemainingEpochsUntilClaim \
        --arguments $1
}

getCreatorLastWithdrawalEpoch() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function getCreatorLastWithdrawalEpoch \
        --arguments $1
}




#-------- SHELL EXECUTED FUNCTIONS --------------

upgrade

#------------------------------------------------
