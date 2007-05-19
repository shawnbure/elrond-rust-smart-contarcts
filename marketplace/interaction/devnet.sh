MY_WALLET_PEM="~/Wallets/WalletKey.pem"
MY_OTHER_WALLET_PEM="OtherWalletKey.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
WASM="../output/marketplace.wasm"

CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqm4dmwyxc5fsj49z3jcu9h08azjrcf60kt9uspxs483"
CONTRACT_ADDRESS_HEX="0x00000000000000000500dd5bb710d8a2612a945196385bbcfd148784e9f65979"
MY_OTHER_ADDRESS="erd13rp6j2fg5wcqdztuwtt5z2n0ls8u0rplnqhyxd676mjtxd09fk7seef9ug"
MY_ADDRESS="erd17s2pz8qrds6ake3qwheezgy48wzf7dr5nhdpuu2h4rr4mt5rt9ussj7xzh"

deploy() {
    erdpy --verbose contract deploy --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=200000000 \
        --arguments 0xfa 0x03e8 0x038D7EA4C68000 0x52B7D2DCC80CD2E4000000 0x1e \
        --send || return
}

upgrade() {
    erdpy --verbose contract upgrade ${CONTRACT_ADDRESS} --recall-nonce \
        --bytecode=${WASM} \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=100000000 \
        --arguments 0xfa 0x03e8 0x038D7EA4C68000 0x52B7D2DCC80CD2E4000000 \
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
        --value=$1
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function deposit \
        --arguments $1 $2 \
        --gas-limit=100000000 \
        --send || return
}

# $1 amount to deposit
withdraw() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function withdraw \
        --arguments $1 $2 \
        --gas-limit=100000000 \
        --send || return
}
