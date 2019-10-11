use crate::{PyAccountId, PyPublicKey, PyDuration};
use hedera::transaction::TransactionCryptoCreate;

def_transaction!(TransactionCryptoCreate() {
    pub fn key(&PyPublicKey);
    pub fn initial_balance(u64);
    pub fn proxy_account(&PyAccountId);
    pub fn auto_renew_period(&PyDuration);
    pub fn send_record_threshold(i64);
    pub fn receive_record_threshold(i64);
    pub fn receiver_signature_required(bool);
});
