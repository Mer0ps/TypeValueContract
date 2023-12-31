// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           58
// Async Callback (empty):               1
// Total number of exported functions:  60

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    typevalue
    (
        insertI8
        insertI16
        insertI32
        insertI64
        insertBigInt
        insertU8
        insertU16
        insertU32
        insertU64
        insertBigUInt
        insertManagedAddress
        insertTokenIdentifier
        insertEgldOrEsdtTokenIdentifier
        getI8
        getI16
        getI32
        getI64
        getBigInt
        getU8
        getU16
        getU32
        getU64
        getBigUInt
        getManagedAddress
        getTokenIdentifier
        getEgldOrEsdtTokenIdentifier
        insertMyStruct
        insertMyStruct2
        insertManagedVecMyStruct2
        getMyStruct
        getMyStruct2
        getManagedVecMyStruct2
        insertManagedVecManagedBuffer
        insertManagedVecI64
        getManagedVecManagedBuffer
        getManagedVecI64
        insertVariadicManagedAddress
        insertVariadicTokenIdentifierManagedAddress
        getVariadicManagedAddress
        getVariadicTokenIdentifierManagedAddress
        insertTupleI64ValueBooleanValue
        insertTupleI64ValueBooleanValueManagedBuffer
        insertTupleManagedVecI64BooleanValue
        getTupleI64ValueBooleanValue
        getTupleI64ValueBooleanValueManagedBuffer
        getTupleManagedVecI64BooleanValue
        insertOptionalValueU8
        insertOptionalValueEmptyU8
        insertOptionU8
        insertOptionEmptyU8
        getOptionalValueEmptyU64
        getOptionalValueU64
        getOptionValueEmptyU64
        getOptionValueU64
        getOptionalU8
        getOptionalEmptyU8
        getOptionU8
        getOptionEmptyU8
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
