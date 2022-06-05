////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    staking
    (
        addStakableTokenIdentifier
        disburseRewards
        geStakedAddressNFTs
        geStakedNFTInfo
        geStakedPool
        geStakedTokenIndentifier
        getAddressRole
        getGlobalOpOngoing
        getLastPayoutDatetime
        getStakedNFTRolloverBalance
        getStakedNFTStakedDatetime
        getStakingRewardBalance
        getVersion
        isNFTStaked
        qualifiedStakedNFTsForRewards
        redeemStakingRewards
        removeStakableTokenIdentifier
        setAddressRole
        setLastPayoutDatetime
        setLastPayoutDatetimeToBlockTimestamp
        stakeAddressNFT
        startGlobalOp
        stopGlobalOp
        test_difference_lastpayoutdatetime_vs_currenttime
        test_get_currenttime
        test_get_new_lastpayoutdatetime
        test_reset_lastpayoutdatetime
        test_set_address_reward
        test_time_block_calculations
        unstakeAddressNFT
    )
}

elrond_wasm_node::wasm_empty_callback! {}
