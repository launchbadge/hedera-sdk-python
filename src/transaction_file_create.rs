use crate::PyPublicKey;
use hedera::transaction::TransactionFileCreate;

//  TODO: once DateTime shenanigans are working
//  `fn expires_at(&PyDateTime);`
def_transaction!(TransactionFileCreate() {
    fn key(&PyPublicKey);
    fn contents(Vec<u8>);
});
