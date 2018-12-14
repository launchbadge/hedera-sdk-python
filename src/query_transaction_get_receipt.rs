use crate::PyTransactionReceipt;
use hedera::{query::QueryTransactionGetReceipt, TransactionId};

def_query!(QueryTransactionGetReceipt(TransactionId) -> PyTransactionReceipt);
