multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait VariadicValue{

    
    #[only_owner]
    #[endpoint(insertVariadicManagedAddress)]
    fn insert_set_mapper_address(&self, value: ManagedAddress){
        self.storage_set_mapper_address().insert(value);
    }

    #[only_owner]
    #[endpoint(insertVariadicTokenIdentifierManagedAddress)]
    fn insert_map_mapper_token_identifier_address(&self, identifier: TokenIdentifier, value: ManagedAddress){
        self.storage_map_mapper_token_identifier_address().insert(identifier, value);
    }

    //Storage

    #[view(getVariadicManagedAddress)]
    #[storage_mapper("storageSetMapperAddress")]
    fn storage_set_mapper_address(&self) -> SetMapper<ManagedAddress>;

    #[view(getVariadicTokenIdentifierManagedAddress)]
    #[storage_mapper("storageMapMapperTokenIdentifierAddress")]
    fn storage_map_mapper_token_identifier_address(&self) -> MapMapper<TokenIdentifier, ManagedAddress>;

}