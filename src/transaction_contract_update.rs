use crate::{PyAccountId, PyFileId, PyPublicKey, PyDuration, PyTimestamp};
use hedera::{transaction::TransactionContractUpdate, ContractId};

def_transaction!(TransactionContractUpdate(ContractId){
    pub fn expires_at(&PyTimestamp);
    pub fn admin_key(&PyPublicKey);
    pub fn proxy_account(&PyAccountId);
    pub fn auto_renew_period(&PyDuration);
    pub fn file(&PyFileId);
});
