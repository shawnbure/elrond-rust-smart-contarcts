#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Deployer {
    #[init]
    fn init(&self, nft_template_address: ManagedAddress, marketplace_admin: ManagedAddress) {
        self.nft_template_address().set(&nft_template_address);
        self.marketplace_admin().set(&marketplace_admin);
    }

    #[payable("EGLD")]
    #[endpoint(deployNFTTemplateContract)]
    fn deploy_nft_template_contract(
        &self,
        token_id: ManagedBuffer,
        royalties: BigUint,
        token_name_base: ManagedBuffer,
        image_base_uri: ManagedBuffer,
        image_extension: ManagedBuffer,
        price: BigUint,
        max_supply: u16,
        sale_start_timestamp: u64,
        #[var_args] metadata_base_uri_opt: OptionalValue<ManagedBuffer>,
    ) -> SCResult<ManagedAddress> {
        let mut arg_buffer = ManagedArgBuffer::new_empty();
        arg_buffer.push_arg(self.marketplace_admin().get());
        arg_buffer.push_arg(token_id);
        arg_buffer.push_arg(royalties);
        arg_buffer.push_arg(token_name_base);
        arg_buffer.push_arg(image_base_uri);
        arg_buffer.push_arg(image_extension);
        arg_buffer.push_arg(price);
        arg_buffer.push_arg(max_supply);
        arg_buffer.push_arg(sale_start_timestamp);

        let metadata_base_uri = metadata_base_uri_opt.into_option();
        if metadata_base_uri.is_some() {
            arg_buffer.push_arg(metadata_base_uri.unwrap());
        }

        let (new_address, _) = Self::Api::send_api_impl().deploy_from_source_contract(
            self.blockchain().get_gas_left(),
            &BigUint::zero(),
            &self.nft_template_address().get(),
            CodeMetadata::PAYABLE | CodeMetadata::UPGRADEABLE,
            &arg_buffer,
        );

        let caller = self.blockchain().get_caller();
        self.owner_of_contract(&new_address).set(&caller);

        Ok(new_address)
    }

    #[endpoint(changeOwner)]
    fn change_owner(&self, sc_address: ManagedAddress) -> SCResult<()> {
        require!(
            !self.owner_of_contract(&sc_address).is_empty(),
            "not an owned contract"
        );

        let caller = self.blockchain().get_caller();
        require!(
            self.owner_of_contract(&sc_address).get() == caller,
            "not owner of contract"
        );

        self.send()
            .change_owner_address(sc_address.clone(), &caller)
            .execute_on_dest_context_ignore_result();
        Ok(())
    }

    #[only_owner]
    #[endpoint(withdraw)]
    fn withdraw(&self, #[var_args] amount_opt: OptionalArg<BigUint>) {
        let amount = amount_opt.into_option().unwrap_or(
            self.blockchain()
                .get_balance(&self.blockchain().get_sc_address()),
        );
        self.send()
            .direct_egld(&self.blockchain().get_caller(), &amount, &[]);
    }

    #[only_owner]
    #[endpoint(setMarketplaceAddress)]
    fn set_marketplace_admin(&self, sc_address: ManagedAddress) {
        self.marketplace_admin().set(&sc_address);
    }

    #[only_owner]
    #[endpoint(setNftTemplateAddress)]
    fn set_nft_template_address(&self, sc_address: ManagedAddress) {
        self.nft_template_address().set(&sc_address);
    }

    #[view(getMarketplaceAddress)]
    #[storage_mapper("marketplace_admin")]
    fn marketplace_admin(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getNftTemplateAddress)]
    #[storage_mapper("nft_template_address")]
    fn nft_template_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getOwnerOfContract)]
    #[storage_mapper("owner_of_contract")]
    fn owner_of_contract(&self, sc_address: &ManagedAddress) -> SingleValueMapper<ManagedAddress>;
}
