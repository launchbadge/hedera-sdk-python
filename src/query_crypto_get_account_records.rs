use crate::PyTransactionRecord;
use hedera::{query::QueryCryptoGetAccountRecords, AccountId};

def_query!(QueryCryptoGetAccountRecords(AccountId) -> Vec<PyTransactionRecord>);
