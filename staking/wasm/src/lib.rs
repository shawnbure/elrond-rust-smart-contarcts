#![no_std]

elrond_wasm_node::wasm_endpoints! {
    staking
    (
        setLastPayoutDatetime
        setLastPayoutDatetimeToBlockTimestamp

        setAddressRole

        addStakableTokenIdentifier
        removeStakableTokenIdentifier

        isNFTStaked
        getStakedNFTStakedDatetime
        getStakedNFTRolloverBalance
        stakeAddressNFT
        unstakeAddressNFT
        getStakingRewardBalance
        redeemStakingRewards

        qualifiedStakedNFTsForRewards
        disburseRewards

        geStakedPool
        geStakedAddressNFTs
        geStakedNFTInfo
        getLastPayoutDatetime
        geStakedTokenIndentifier
        getAddressRole
        getVersion      





        test_set_address_reward
        test_reset_lastpayoutdatetime
        test_get_currenttime
        test_difference_lastpayoutdatetime_vs_currenttime   
        test_time_block_calculations
        test_get_new_lastpayoutdatetime
        
        //test_reward_split
        //test_marketplace_fee_splits
    )
}
