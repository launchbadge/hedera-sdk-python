use crate::{timestamp::PyTimestamp, PyPublicKey};
use hedera::transaction::TransactionFileCreate;

def_transaction!(TransactionFileCreate() {
    pub fn key(&PyPublicKey);
    pub fn contents(Vec<u8>);
    pub fn expires_at(&PyTimestamp);
});
