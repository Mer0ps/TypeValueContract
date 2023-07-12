multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ListValue{

    #[only_owner]
    #[endpoint(insertManagedVecManagedBuffer)]
    fn insert_managed_vec_managed_buffer(&self, value: ManagedVec<ManagedBuffer>){
        self.storage_managed_vec_managed_buffer().set(value);
    }

    #[only_owner]
    #[endpoint(insertManagedVecI64)]
    fn insert_managed_vec_i64(&self, value: ManagedVec<i64>){
        self.storage_managed_vec_i64().set(value);
    }

    //Storage

    #[view(getManagedVecManagedBuffer)]
    #[storage_mapper("storageManagedVecManagedBuffer")]
    fn storage_managed_vec_managed_buffer(&self) -> SingleValueMapper<ManagedVec<ManagedBuffer>>;

    #[view(getManagedVecI64)]
    #[storage_mapper("storageManagedVecI64")]
    fn storage_managed_vec_i64(&self) -> SingleValueMapper<ManagedVec<i64>>;
}