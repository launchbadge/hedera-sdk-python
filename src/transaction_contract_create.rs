use crate::{PyAccountId, PyFileId, PyPublicKey};
use hedera::transaction::TransactionContractCreate;

def_transaction!(TransactionContractCreate() {
    fn file(&PyFileId);

    fn gas(i64);

    fn admin_key(&PyPublicKey);

    fn initial_balance(i64);

    fn proxy_account(&PyAccountId);

    fn proxy_fraction(i32);

    // TODO: fn auto_renew_period(&PyDuration);

    fn constructor_parameters(Vec<u8>);
});
