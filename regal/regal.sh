############### DEVELOPMENT ################################

MY_WALLET_PEM="cp-new-wallet.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
TEMPLATE_CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqj5qg3p4uq3nup4mjrmz9c2w24k00fajty4wsh65lq7"
DAO_WALLET_ADDRESS_BECH32="54d029b2a71cfec4497cc56fecb299534847aee2465cd2d32d9e55c1dc861b96"

############### DEVELOPMENT ################################


############### PRODUCTION ################################

#MY_WALLET_PEM="dao.pem"
#PROXY="https://gateway.elrond.com"
#CHAIN_ID="1"
#TEMPLATE_CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqjc2aaea3mzr375nqnulya6x4lxl0qrp6y4wslghcz6"
#DAO_WALLET_ADDRESS_BECH32="0x6b3d87c350a9fc286199e186de9e479dc9a2b58808083b7c419afbf358082319" 

############### PRODUCTION ################################

HEX_PREFIXED_BECH32_ADDRESS="0x"
HEX_PREFIXED_BECH32_ADDRESS+=${DAO_WALLET_ADDRESS_BECH32}

updateBuyerWhitelistCheckOn() {
    erdpy --verbose contract call ${TEMPLATE_CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function updateBuyerWhitelistCheck \
        --arguments 1 \
        --gas-limit=20000000 \
        --send || return
        
        getResults "getBuyerWhiteListCheck"
        
}

setZeroPrice() {
    erdpy --verbose contract call ${TEMPLATE_CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function setPrice \
        --arguments 0 \
        --gas-limit=20000000 \
        --send || return
        
        getResults "getPrice"
}

createBuyerAddress() {
    erdpy --verbose contract call ${TEMPLATE_CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function createBuyerAddress \
        --arguments 0 1000 ${HEX_PREFIXED_BECH32_ADDRESS} \
        --gas-limit=20000000 \
        --send || return
        
        getResults "getBuyCount" ${HEX_PREFIXED_BECH32_ADDRESS}
        getResults "getBuyLimit" ${HEX_PREFIXED_BECH32_ADDRESS}
}

setInitalSalePrice() {
    erdpy --verbose contract call ${TEMPLATE_CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function setPrice \
        --arguments 850000000000000000 \
        --gas-limit=20000000 \
        --send || return
        
        getResults "getPrice"
}

updateBuyerWhitelistCheckOff() {
    erdpy --verbose contract call ${TEMPLATE_CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function updateBuyerWhitelistCheck \
        --arguments 0 \
        --gas-limit=20000000 \
        --send || return
        
        getResults "getBuyerWhiteListCheck"
}

setFinalSalePrice() {
    erdpy --verbose contract call ${TEMPLATE_CONTRACT_ADDRESS} --recall-nonce \
        --pem=${MY_WALLET_PEM} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function setPrice \
        --arguments 1000000000000000000 \
        --gas-limit=20000000 \
        --send || return
        
        getResults "getPrice"
}

getResults() {

	echo ;
    echo "Wait 5 seconds for the chain and then check the set result."    
    sleep 5
	echo ; 
    echo $1

	if [[ ! $2 ]] # args var is $2 - if is not set or it is set to an empty string, then do this
	then 
    
		erdpy --verbose contract query ${TEMPLATE_CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function $1 \
		|| return
	
    else

    	erdpy --verbose contract query ${TEMPLATE_CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function $1 \
        --arguments $2 \
		|| return
    fi
}

"$@"