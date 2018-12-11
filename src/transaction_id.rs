use crate::errors::PyValueError;
use derive_more::From;
use hedera::TransactionId;
use pyo3::prelude::*;
use std::str::FromStr;

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
        obj.init(|_| Self { inner: id })
    }
}

def_str!(PyTransactionId);
