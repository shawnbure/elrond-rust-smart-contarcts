MY_WALLET_PEM="../../dev-wallet-owner.pem"                  
MY_OTHER_WALLET_PEM="../../dev-extra-wallet-owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
WASM="../output/staking.wasm"
VERSION="0.0.1"
VERSION_HEX=0x302E302E31
#SETUP THIS AFTER DEPLOYMENT
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqvqxzny609xfhvhnp4stva370jtk8p0xny4wszmlt30"           #after deploying, the the contract address
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
        --arguments ${VERSION_HEX} \
        --send || return
}


#-------- SHELL EXECUTED FUNCTIONS --------------

deploy

#------------------------------------------------
