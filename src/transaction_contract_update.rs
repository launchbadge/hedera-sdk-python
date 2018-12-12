use crate::{PyPublicKey, PyAccountId, PyFileId};
use hedera::{transaction::TransactionContractUpdate, ContractId};

def_transaction!(TransactionContractUpdate(ContractId){
    // TODO: fn expiration_time(DateTime<Utc>);
    fn admin_key(&PyPublicKey);
    fn proxy_account(&PyAccountId);
    // TODO: fn auto_renew_period(Duration);
    fn file(&PyFileId);
});
