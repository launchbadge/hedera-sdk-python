use super::errors::PyException;
use hedera::{
    query::{Query, QueryCryptoGetAccountBalance},
    AccountId, Client,
};
use pyo3::prelude::*;

#[pyclass(name = QueryCryptoGetAccountBalance)]
pub struct PyQueryCryptoGetAccountBalance {
    inner: Query<u64>,
}

impl PyQueryCryptoGetAccountBalance {
    pub(crate) fn new(client: &Client, account: AccountId) -> Self {
        Self {
            inner: QueryCryptoGetAccountBalance::new(client, account),
        }
    }
}

#[pymethods]
impl PyQueryCryptoGetAccountBalance {
    pub fn get(&mut self) -> PyResult<u64> {
        self.inner.get().map_err(PyException)
    }

    pub fn cost(&mut self) -> PyResult<u64> {
        self.inner.cost().map_err(PyException)
    }
}
