multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ListValue{

    #[endpoint]
    fn insert_set_mapper_address(&self, value: ManagedAddress){
        self.storage_set_mapper_address().insert(value);
    }

    #[endpoint]
    fn insert_map_mapper_token_identifier_address(&self, identifier: TokenIdentifier, value: ManagedAddress){
        self.storage_map_mapper_token_identifier_address().insert(identifier, value);
    }

    #[view(StorageSetMapperAddress)]
    #[storage_mapper("StorageSetMapperAddress")]
    fn storage_set_mapper_address(&self) -> SetMapper<ManagedAddress>;

    #[view(StorageMapMapperTokenIdentifierAddress)]
    #[storage_mapper("StorageMapMapperTokenIdentifierAddress")]
    fn storage_map_mapper_token_identifier_address(&self) -> MapMapper<TokenIdentifier, ManagedAddress>;
}