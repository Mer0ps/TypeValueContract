multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct MyStruct<M: ManagedTypeApi> {
  name: ManagedBuffer<M>,
  long_value: u64,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, ManagedVecItem)]
pub struct MyStruct2<M: ManagedTypeApi> {
  name: ManagedBuffer<M>,
  long_value: u64,
  list_bool: ManagedVec<M, bool>
}

#[multiversx_sc::module]
pub trait StructValue{

    #[only_owner]
    #[endpoint(insertMyStruct)]
    fn insert_my_struct(&self, value: MyStruct<Self::Api>){
        self.storage_my_struct().set(value);
    }

    #[only_owner]
    #[endpoint(insertMyStruct2)]
    fn insert_my_struct2(&self, value: MyStruct2<Self::Api>){
        self.storage_my_struct2().set(value);
    }

    #[only_owner]
    #[endpoint(insertManagedVecMyStruct2)]
    fn insert_managed_vec_my_struct2(&self, value: ManagedVec<MyStruct2<Self::Api>>){
        self.storage_managed_vec_my_struct2().set(value);
    }

    //Storage

    #[view(getMyStruct)]
    #[storage_mapper("storageMyStruct")]
    fn storage_my_struct(&self) -> SingleValueMapper<MyStruct<Self::Api>>;

    #[view(getMyStruct2)]
    #[storage_mapper("storageMyStruct2")]
    fn storage_my_struct2(&self) -> SingleValueMapper<MyStruct2<Self::Api>>;

    #[view(getManagedVecMyStruct2)]
    #[storage_mapper("storageManagedVecMyStruct2")]
    fn storage_managed_vec_my_struct2(&self) -> SingleValueMapper<ManagedVec<MyStruct2<Self::Api>>>;
}