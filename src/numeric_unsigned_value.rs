multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait NumericUnsignedValue{

    //Endpoints 
    #[endpoint]
    fn insert_u8(&self, value: u8){
        self.storage_u8().set(value);
    }

    #[endpoint]
    fn insert_u16(&self, value: u16){
        self.storage_u16().set(value);
    }

    #[endpoint]
    fn insert_u32(&self, value: u32){
        self.storage_u32().set(value);
    }

    #[endpoint]
    fn insert_u64(&self, value: u64){
        self.storage_u64().set(value);
    }

    #[endpoint]
    fn insert_big_uint(&self, value: BigUint){
        self.storage_big_uint().set(value);
    }

    //Storage 

    #[view(storageU8)]
    #[storage_mapper("storageU8")]
    fn storage_u8(&self) -> SingleValueMapper<u8>;

    #[view(storageU16)]
    #[storage_mapper("storageU16")]
    fn storage_u16(&self) -> SingleValueMapper<u16>;

    #[view(storageU32)]
    #[storage_mapper("storageU32")]
    fn storage_u32(&self) -> SingleValueMapper<u32>;

    #[view(storageU64)]
    #[storage_mapper("storageU64")]
    fn storage_u64(&self) -> SingleValueMapper<u64>;

    #[view(storageBigUint)]
    #[storage_mapper("storageBigUint")]
    fn storage_big_uint(&self) -> SingleValueMapper<BigUint>;


}