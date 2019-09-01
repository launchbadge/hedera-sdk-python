use crate::{timestamp::PyTimestamp, PyPublicKey};
use hedera::transaction::TransactionFileCreate;

//  TODO: once DateTime shenanigans are working
//  `pub fn expires_at(&PyTimestamp);`
def_transaction!(TransactionFileCreate() {
    pub fn key(&PyPublicKey);
    pub fn contents(Vec<u8>);
    pub fn expires_at(&PyTimestamp);
});
