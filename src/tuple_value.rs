multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait TupleValue{

    #[endpoint]
    fn insert_tuple_i64_bool(&self, value: (i64, bool)){
        self.storage_tuple_i64_bool().set(value);
    }

    #[view(storageTupleI64Bool)]
    #[storage_mapper("storageTupleI64Bool")]
    fn storage_tuple_i64_bool(&self) -> SingleValueMapper<(i64, bool)>;

    #[endpoint]
    fn insert_tuple_i64_bool_managed_buffer(&self, value: (i64, bool, ManagedBuffer)){
        self.storage_tuple_i64_bool_managed_buffer().set(value);
    }

    #[view(storageTupleI64BoolManagedBuffer)]
    #[storage_mapper("storageTupleI64BoolManagedBuffer")]
    fn storage_tuple_i64_bool_managed_buffer(&self) -> SingleValueMapper<(i64, bool, ManagedBuffer)>;

    #[endpoint]
    fn insert_tuple_managed_vec_i64_bool(&self, value: (ManagedVec<i64>, bool)){
        self.storage_tuple_managed_vec_i64_bool().set(value);
    }

    #[view(storageTupleManagedVecI64Bool)]
    #[storage_mapper("storageTupleManagedVecI64Bool")]
    fn storage_tuple_managed_vec_i64_bool(&self) -> SingleValueMapper<(ManagedVec<i64>, bool)>;

}