use hedera::{transaction::TransactionContractCall, ContractId};

def_transaction!(TransactionContractCall(ContractId) {
    pub fn gas(i64);
    pub fn amount(i64);
    pub fn function_parameters(Vec<u8>);
});
