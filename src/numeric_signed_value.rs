multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait NumericSignedValue{

    //Endpoints 
    #[endpoint]
    fn insert_i8(&self, value: i8){
        self.storage_i8().set(value);
    }

    #[endpoint]
    fn insert_i16(&self, value: i16){
        self.storage_i16().set(value);
    }

    #[endpoint]
    fn insert_i32(&self, value: i32){
        self.storage_i32().set(value);
    }

    #[endpoint]
    fn insert_i64(&self, value: i64){
        self.storage_i64().set(value);
    }

    #[endpoint]
    fn insert_big_int(&self, value: BigInt){
        self.storage_big_int().set(value);
    }

    //Storage 

    #[view(storageI8)]
    #[storage_mapper("storageI8")]
    fn storage_i8(&self) -> SingleValueMapper<i8>;

    #[view(storageI16)]
    #[storage_mapper("storageI16")]
    fn storage_i16(&self) -> SingleValueMapper<i16>;

    #[view(storageI32)]
    #[storage_mapper("storageI32")]
    fn storage_i32(&self) -> SingleValueMapper<i32>;

    #[view(storageI64)]
    #[storage_mapper("storageI64")]
    fn storage_i64(&self) -> SingleValueMapper<i64>;

    #[view(storageBigInt)]
    #[storage_mapper("storageBigInt")]
    fn storage_big_int(&self) -> SingleValueMapper<BigInt>;

}