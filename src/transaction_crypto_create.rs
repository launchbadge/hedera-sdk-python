use crate::{PyAccountId, PyPublicKey};
use hedera::transaction::TransactionCryptoCreate;

def_transaction!(TransactionCryptoCreate() {
    fn key(&PyPublicKey);
    fn initial_balance(u64);
    fn proxy_account(&PyAccountId);
    // TODO: fn auto_renew_period(Duration);
    fn send_record_threshold(i64);
    fn receive_record_threshold(i64);
    fn receiver_signature_required(bool);
});
