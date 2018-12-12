use crate::{PyAccountId, PyPublicKey};
use hedera::{transaction::TransactionCryptoUpdate, AccountId};

def_transaction!(TransactionCryptoUpdate(AccountId) {
    fn key(&PyPublicKey);
    fn proxy_account(&PyAccountId);
    fn proxy_fraction(i32);
    fn send_record_threshold(u64);
    fn receive_record_threshold(u64);
//    fn auto_renew_period(&PyDuration);
//    fn expiration_time(&PyDateTime);
});
