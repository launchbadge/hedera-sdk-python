use crate::PyPublicKey;
use hedera::{transaction::TransactionFileUpdate, FileId};

def_transaction!(TransactionFileUpdate(FileId) {
//  fn expires_at(PyDateTime);
    fn key(&PyPublicKey);
    fn contents(Vec<u8>);
});
