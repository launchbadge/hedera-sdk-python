use crate::PyAccountId;
use hedera::{transaction::TransactionCryptoDelete, AccountId};

def_transaction!(TransactionCryptoDelete(AccountId) {
    fn transfer_to(&PyAccountId);
});
