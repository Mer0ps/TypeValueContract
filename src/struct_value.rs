multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct MyStruct<M: ManagedTypeApi> {
  name: ManagedBuffer<M>,
  long_value: u64,
}

#[multiversx_sc::module]
pub trait StructValue{

    #[endpoint]
    fn insert_my_struct(&self, value: MyStruct<Self::Api>){
        self.storage_my_struct().set(value);
    }

    #[view(storageMyStruct)]
    #[storage_mapper("storageMyStruct")]
    fn storage_my_struct(&self) -> SingleValueMapper<MyStruct<Self::Api>>;

}