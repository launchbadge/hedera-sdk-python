use super::{errors::PyException, transaction_receipt::PyTransactionReceipt};
use hedera::{
    query::{Query, QueryGetTransactionReceipt},
    Client, TransactionId, TransactionReceipt,
};
use pyo3::prelude::*;

#[pyclass(name = QueryGetTransactionReceipt)]
pub struct PyQueryGetTransactionReceipt {
    inner: Query<QueryGetTransactionReceipt>,
}

impl PyQueryGetTransactionReceipt {
    pub(crate) fn new(client: &Client, transaction: TransactionId) -> Self {
        Self {
            inner: QueryGetTransactionReceipt::new(client, transaction),
        }
    }
}

#[pymethods]
impl PyQueryGetTransactionReceipt {
    pub fn get(&mut self) -> PyResult<PyTransactionReceipt> {
        self.inner.get().map(Into::into).map_err(PyException)
    }

    pub fn cost(&mut self) -> PyResult<u64> {
        self.inner.cost().map_err(PyException)
    }
}
