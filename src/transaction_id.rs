use crate::{errors::PyValueError, PyAccountId, PyDateTime};
use derive_more::From;
use hedera::TransactionId;
use pyo3::prelude::*;
use std::str::FromStr;
use try_from::TryInto;

#[pyclass(name = TransactionId)]
#[derive(From)]
pub struct PyTransactionId {
    pub(crate) inner: TransactionId,
}

#[pymethods]
impl PyTransactionId {
    #[new]
    pub fn __new__(obj: &PyRawObject, s: &str) -> PyResult<()> {
        let id = TransactionId::from_str(s).map_err(PyValueError)?;
        obj.init(|| Self { inner: id })
    }

    #[getter]
    pub fn account_id(&self) -> PyResult<PyAccountId> {
        Ok(self.inner.account_id.into())
    }

    #[getter]
    pub fn transaction_valid_start(&self) -> PyResult<PyDateTime> {
        self.inner.transaction_valid_start.try_into()
    }
}

def_str!(PyTransactionId);
