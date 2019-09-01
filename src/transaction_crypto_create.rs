use crate::{PyAccountId, PyPublicKey};
use hedera::transaction::TransactionCryptoCreate;

def_transaction!(TransactionCryptoCreate() {
    pub fn key(&PyPublicKey);
    pub fn initial_balance(u64);
    pub fn proxy_account(&PyAccountId);
    // TODO: pub fn auto_renew_period(Duration);
    pub fn send_record_threshold(i64);
    pub fn receive_record_threshold(i64);
    pub fn receiver_signature_required(bool);
});
