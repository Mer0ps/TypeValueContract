multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait TupleValue{

    #[only_owner]
    #[endpoint(insertTupleI64ValueBooleanValue)]
    fn insert_tuple_i64_bool(&self, value: (i64, bool)){
        self.storage_tuple_i64_bool().set(value);
    }

    #[only_owner]
    #[endpoint(insertTupleI64ValueBooleanValueManagedBuffer)]
    fn insert_tuple_i64_bool_managed_buffer(&self, value: (i64, bool, ManagedBuffer)){
        self.storage_tuple_i64_bool_managed_buffer().set(value);
    }

    #[only_owner]
    #[endpoint(insertTupleManagedVecI64BooleanValue)]
    fn insert_tuple_managed_vec_i64_bool(&self, value: (ManagedVec<i64>, bool)){
        self.storage_tuple_managed_vec_i64_bool().set(value);
    }

    //Storage

    #[view(getTupleI64ValueBooleanValue)]
    #[storage_mapper("storageTupleI64Bool")]
    fn storage_tuple_i64_bool(&self) -> SingleValueMapper<(i64, bool)>;

    #[view(getTupleI64ValueBooleanValueManagedBuffer)]
    #[storage_mapper("storageTupleI64BoolManagedBuffer")]
    fn storage_tuple_i64_bool_managed_buffer(&self) -> SingleValueMapper<(i64, bool, ManagedBuffer)>;

    #[view(getTupleManagedVecI64BooleanValue)]
    #[storage_mapper("storageTupleManagedVecI64Bool")]
    fn storage_tuple_managed_vec_i64_bool(&self) -> SingleValueMapper<(ManagedVec<i64>, bool)>;

}