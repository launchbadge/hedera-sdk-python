use crate::{PyAccountId, PyPublicKey};
use hedera::{transaction::TransactionCryptoUpdate, AccountId};

def_transaction!(TransactionCryptoUpdate(AccountId) {
    pub fn key(&PyPublicKey);
    pub fn proxy_account(&PyAccountId);
    pub fn send_record_threshold(u64);
    pub fn receive_record_threshold(u64);
//    pub fn auto_renew_period(&PyDuration);
//    pub fn expiration_time(&PyTimestamp);
});
