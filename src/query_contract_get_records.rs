use crate::PyTransactionRecord;
use hedera::{query::QueryContractGetRecords, ContractId};

def_query!(QueryContractGetRecords(ContractId) -> Vec<PyTransactionRecord>);
