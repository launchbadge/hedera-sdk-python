use crate::{PyAccountId};
use hedera::transaction::TransactionContractDelete;
use hedera::ContractId;

def_transaction!(TransactionContractDelete(ContractId) {
    pub fn obtainer_account(&PyAccountId);
});
