use crate::PyAccountId;
use hedera::{transaction::TransactionCryptoDelete, AccountId};

def_transaction!(TransactionCryptoDelete(AccountId) {
    pub fn transfer_to(&PyAccountId);
});
