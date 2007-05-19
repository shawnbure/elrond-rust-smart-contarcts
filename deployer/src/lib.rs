#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Deployer {
    #[init]
    fn init(&self, nft_template_address: ManagedAddress) {
        self.nft_template_address().set(&nft_template_address);
    }

    #[payable("EGLD")]
    #[endpoint(deployNFTTemplateContract)]
    fn deploy_nft_template_contract(
        &self,
        token_id: BoxedBytes,
        royalties: BigUint,
        token_name_base: BoxedBytes,
        image_base_uri: BoxedBytes,
        image_extension: BoxedBytes,
        price: BigUint,
        max_supply: u16,
        sale_start_timestamp: u64,
        #[var_args] metadata_base_uri_opt: OptionalArg<BoxedBytes>,
    ) -> SCResult<ManagedAddress> {
        let mut arg_buffer = ManagedArgBuffer::new_empty(self.type_manager());
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

        let (new_address, _) = self.raw_vm_api().deploy_from_source_contract(
            self.blockchain().get_gas_left(),
            &BigUint::zero(),
            &self.nft_template_address().get(),
            CodeMetadata::PAYABLE,
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
    fn withdraw(&self) {
        self.send().direct_egld(
            &self.blockchain().get_caller(),
            &self
                .blockchain()
                .get_balance(&self.blockchain().get_sc_address()),
            &[],
        );
    }

    #[view(getNftTemplateAddress)]
    #[storage_mapper("nft_template_address")]
    fn nft_template_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getOwnerOfContract)]
    #[storage_mapper("owner_of_contract")]
    fn owner_of_contract(&self, sc_address: &ManagedAddress) -> SingleValueMapper<ManagedAddress>;
}
