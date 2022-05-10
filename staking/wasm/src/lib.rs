#![no_std]

elrond_wasm_node::wasm_endpoints! {
    staking
    (
        setLastPayoutDatetime
        setLastPayoutDatetimeToBlockTimestamp
        setAdminAddress  

        addStakableTokenIdentifier
        removeStakableTokenIdentifier

        isNFTStaked
        getStakedNFTStakedDatetime
        getStakedNFTRolloverBalance
        stakeAddressNFT
        unstakeAddressNFT
        getStakingRewardBalance
        redeemStakingRewards
        disburseRewards

        geStakedPool
        geStakedAddressNFTs
        geStakedNFTInfo
        getLastPayoutDatetime
        geStakedTokenIndentifier
        getAdminAddress
        getVersion      



        test_set_address_reward
    )
}
