#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod simple_value;
mod struct_value;
mod list_value;
mod variadic_value;
mod tuple_value;
mod option_value;

#[multiversx_sc::contract]
pub trait TypeValue:
    simple_value::SimpleValue
    + struct_value::StructValue
    + list_value::ListValue
    + variadic_value::VariadicValue
    + tuple_value::TupleValue 
    + option_value::OptionValue {

    #[init]
    fn init(&self) {
    }    
}
