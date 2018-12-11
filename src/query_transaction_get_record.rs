use crate::PyTransactionRecord;
use hedera::{query::QueryTransactionGetRecord, TransactionId};

def_query!(QueryTransactionGetRecord(TransactionId) -> PyTransactionRecord);
