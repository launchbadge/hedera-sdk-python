use crate::errors::PyValueError;
use hedera::TransactionId;
use pyo3::prelude::*;
use std::str::FromStr;

#[pyclass(name = TransactionId)]
pub struct PyTransactionId {
    pub(crate) inner: TransactionId,
}

#[pymethods]
impl PyTransactionId {
    #[new]
    pub fn __new__(obj: &PyRawObject, id: &str) -> PyResult<()> {
        let id = TransactionId::from_str(id).map_err(PyValueError)?;
        obj.init(|_| Self { inner: id })
    }
}

def_str!(PyTransactionId);
