use hedera::{transaction::TransactionContractCall, ContractId};

def_transaction!(TransactionContractCall(ContractId) {
    fn gas(i64);
    fn amount(i64);
    fn function_parameters(Vec<u8>);
});
