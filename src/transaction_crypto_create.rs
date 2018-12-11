use crate::PyPublicKey;
use hedera::transaction::{TransactionCryptoCreate};

def_transaction!(TransactionCryptoCreate() {
    fn key(&PyPublicKey);
    fn initial_balance(u64);
});
