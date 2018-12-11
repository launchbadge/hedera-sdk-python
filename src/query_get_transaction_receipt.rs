use crate::PyTransactionReceipt;
use hedera::{query::QueryGetTransactionReceipt, TransactionId};

def_query!(QueryGetTransactionReceipt(TransactionId) -> PyTransactionReceipt);
