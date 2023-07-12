multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait SimpleValue{

    #[only_owner]
    #[endpoint(insertI8)]
    fn insert_i8(&self, value: i8){
        self.storage_i8().set(value);
    }

    #[only_owner]
    #[endpoint(insertI16)]
    fn insert_i16(&self, value: i16){
        self.storage_i16().set(value);
    }

    #[only_owner]
    #[endpoint(insertI32)]
    fn insert_i32(&self, value: i32){
        self.storage_i32().set(value);
    }

    #[only_owner]
    #[endpoint(insertI64)]
    fn insert_i64(&self, value: i64){
        self.storage_i64().set(value);
    }

    #[only_owner]
    #[endpoint(insertBigInt)]
    fn insert_big_int(&self, value: BigInt){
        self.storage_big_int().set(value);
    }

    #[only_owner]
    #[endpoint(insertU8)]
    fn insert_u8(&self, value: u8){
        self.storage_u8().set(value);
    }

    #[only_owner]
    #[endpoint(insertU16)]
    fn insert_u16(&self, value: u16){
        self.storage_u16().set(value);
    }

    #[only_owner]
    #[endpoint(insertU32)]
    fn insert_u32(&self, value: u32){
        self.storage_u32().set(value);
    }

    #[only_owner]
    #[endpoint(insertU64)]
    fn insert_u64(&self, value: u64){
        self.storage_u64().set(value);
    }

    #[only_owner]
    #[endpoint(insertBigUInt)]
    fn insert_big_uint(&self, value: BigUint){
        self.storage_big_uint().set(value);
    }

    #[only_owner]
    #[endpoint(insertManagedAddress)]
    fn insert_address(&self, value: ManagedAddress){
        self.storage_managed_address().set(value);
    }

    #[only_owner]
    #[endpoint(insertTokenIdentifier)]
    fn insert_token_identifier(&self, value: TokenIdentifier){
        self.storage_token_identifier().set(value);
    }

    #[only_owner]
    #[endpoint(insertEgldOrEsdtTokenIdentifier)]
    fn insert_egld_or_esdt_token_identifier(&self, value: EgldOrEsdtTokenIdentifier){
        self.storage_egld_or_esdt_token_identifier().set(value);
    }

    //Storage 

    #[view(getI8)]
    #[storage_mapper("storageI8")]
    fn storage_i8(&self) -> SingleValueMapper<i8>;

    #[view(getI16)]
    #[storage_mapper("storageI16")]
    fn storage_i16(&self) -> SingleValueMapper<i16>;

    #[view(getI32)]
    #[storage_mapper("storageI32")]
    fn storage_i32(&self) -> SingleValueMapper<i32>;

    #[view(getI64)]
    #[storage_mapper("storageI64")]
    fn storage_i64(&self) -> SingleValueMapper<i64>;

    #[view(getBigInt)]
    #[storage_mapper("storageBigInt")]
    fn storage_big_int(&self) -> SingleValueMapper<BigInt>;

    #[view(getU8)]
    #[storage_mapper("storageU8")]
    fn storage_u8(&self) -> SingleValueMapper<u8>;

    #[view(getU16)]
    #[storage_mapper("storageU16")]
    fn storage_u16(&self) -> SingleValueMapper<u16>;

    #[view(getU32)]
    #[storage_mapper("storageU32")]
    fn storage_u32(&self) -> SingleValueMapper<u32>;

    #[view(getU64)]
    #[storage_mapper("storageU64")]
    fn storage_u64(&self) -> SingleValueMapper<u64>;

    #[view(getBigUInt)]
    #[storage_mapper("storageBigUint")]
    fn storage_big_uint(&self) -> SingleValueMapper<BigUint>;

    #[view(getManagedAddress)]
    #[storage_mapper("storageManagedAddress")]
    fn storage_managed_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getTokenIdentifier)]
    #[storage_mapper("storageTokenIdentifier")]
    fn storage_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getEgldOrEsdtTokenIdentifier)]
    #[storage_mapper("storageEgldOrEsdtTokenIdentifier")]
    fn storage_egld_or_esdt_token_identifier(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;
}