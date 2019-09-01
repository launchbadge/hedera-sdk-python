use crate::PyPublicKey;
use hedera::{transaction::TransactionFileUpdate, FileId};

def_transaction!(TransactionFileUpdate(FileId) {
//  pub fn expires_at(PyTimestamp);
    pub fn key(&PyPublicKey);
    pub fn contents(Vec<u8>);
});
