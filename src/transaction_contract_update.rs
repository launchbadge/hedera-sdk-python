use crate::{PyAccountId, PyFileId, PyPublicKey};
use hedera::{transaction::TransactionContractUpdate, ContractId};

def_transaction!(TransactionContractUpdate(ContractId){
    // TODO: pub fn expiration_time(DateTime<Utc>);
    pub fn admin_key(&PyPublicKey);
    pub fn proxy_account(&PyAccountId);
    // TODO: pub fn auto_renew_period(Duration);
    pub fn file(&PyFileId);
});
