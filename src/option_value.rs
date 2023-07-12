multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const DEFAULT_OPTIONAL_U8: u8 = 12;
pub const DEFAULT_OPTION_U8: u8 = 8;

#[multiversx_sc::module]
pub trait OptionValue{

    #[only_owner]
    #[endpoint(insertOptionalValueU8)]
    fn insert_optional_value_u8(&self, opt_value: OptionalValue<u8>){

        let u8_value = match opt_value {
            OptionalValue::Some(value) => value,
            OptionalValue::None => DEFAULT_OPTIONAL_U8,
        };

        self.storage_optional_u8().set(u8_value);
    }

    #[only_owner]
    #[endpoint(insertOptionalValueEmptyU8)]
    fn insert_optional_value_empty_u8(&self, opt_value: OptionalValue<u8>){

        let u8_value = match opt_value {
            OptionalValue::Some(value) => value,
            OptionalValue::None => DEFAULT_OPTIONAL_U8,
        };

        self.storage_optional_empty_u8().set(u8_value);
    }

    #[only_owner]
    #[endpoint(insertOptionU8)]
    fn insert_option_u8(&self, opt_value: Option<u8>){

        let value = opt_value.unwrap_or_else(|| DEFAULT_OPTION_U8);

        self.storage_option_u8().set(value);
    }

    #[only_owner]
    #[endpoint(insertOptionEmptyU8)]
    fn insert_option_empty_u8(&self, opt_value: Option<u8>){

        let value = opt_value.unwrap_or_else(|| DEFAULT_OPTION_U8);

        self.storage_option_empty_u8().set(value);
    }


    //View

    #[view(getOptionalValueEmptyU64)]
    fn get_optional_value_empty_u64(&self) -> OptionalValue<u64> {
        return OptionalValue::None;
    }

    #[view(getOptionalValueU64)]
    fn get_optional_value_u64(&self) -> OptionalValue<u64> {
        return OptionalValue::Some(698547526)
    }

    #[view(getOptionValueEmptyU64)]
    fn get_option_value_empty_u64(&self) -> Option<u64> {
        return None;
    }

    #[view(getOptionValueU64)]
    fn get_option_value_u64(&self) -> Option<u64> {
        return Some(365214859)
    }

    //Storage

    #[view(getOptionalU8)]
    #[storage_mapper("storageOptionalU8")]
    fn storage_optional_u8(&self) -> SingleValueMapper<u8>;

    #[view(getOptionalEmptyU8)]
    #[storage_mapper("storageOptionalEmptyU8")]
    fn storage_optional_empty_u8(&self) -> SingleValueMapper<u8>;

    #[view(getOptionU8)]
    #[storage_mapper("storageOptionU8")]
    fn storage_option_u8(&self) -> SingleValueMapper<u8>;

    #[view(getOptionEmptyU8)]
    #[storage_mapper("storageOptionEmptyU8")]
    fn storage_option_empty_u8(&self) -> SingleValueMapper<u8>;


}