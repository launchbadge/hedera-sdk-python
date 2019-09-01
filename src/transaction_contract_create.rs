use crate::{PyAccountId, PyFileId, PyPublicKey};
use hedera::transaction::TransactionContractCreate;

def_transaction!(TransactionContractCreate() {
    pub fn file(&PyFileId);

    pub fn gas(i64);

    pub fn admin_key(&PyPublicKey);

    pub fn initial_balance(i64);

    pub fn proxy_account(&PyAccountId);

    // TODO: pub fn auto_renew_period(&PyDuration);

    pub fn constructor_parameters(Vec<u8>);
});
