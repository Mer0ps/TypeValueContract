#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod numeric_signed_value;
mod numeric_unsigned_value;
mod struct_value;
mod list_value;

#[multiversx_sc::contract]
pub trait TypeValue:
    numeric_signed_value::NumericSignedValue
    + numeric_unsigned_value::NumericUnsignedValue
    + struct_value::StructValue
    + list_value::ListValue {

    #[init]
    fn init(&self) {
    }

    #[endpoint]
    fn insert_address(&self, value: ManagedAddress){
        self.storage_address().set(value);
    }

    #[view(storageAddress)]
    #[storage_mapper("storageAddress")]
    fn storage_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[endpoint]
    fn insert_token_identifier(&self, value: TokenIdentifier){
        self.storage_token_identifier().set(value);
    }

    #[view(storageTokenIdentifier)]
    #[storage_mapper("storageTokenIdentifier")]
    fn storage_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[endpoint]
    fn insert_egld_or_esdt_token_identifier(&self, value: EgldOrEsdtTokenIdentifier){
        self.storage_egld_or_esdt_token_identifier().set(value);
    }

    #[view(storageEgldOrEsdtTokenIdentifier)]
    #[storage_mapper("storageEgldOrEsdtTokenIdentifier")]
    fn storage_egld_or_esdt_token_identifier(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;


    #[endpoint]
    fn insert_managed_vec_managed_buffer(&self, value: ManagedVec<ManagedBuffer>){
        self.storage_managed_vec_managed_buffer().set(value);
    }

    #[view(storageManagedVecManagedBuffer)]
    #[storage_mapper("storageManagedVecManagedBuffer")]
    fn storage_managed_vec_managed_buffer(&self) -> SingleValueMapper<ManagedVec<ManagedBuffer>>;
    
}
