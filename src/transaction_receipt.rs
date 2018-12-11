use derive_more::From;
use hedera::TransactionReceipt;
use pyo3::prelude::*;

#[pyclass(name = TransactionReceipt)]
#[derive(From)]
pub struct PyTransactionReceipt {
    pub(crate) inner: TransactionReceipt,
}

#[pymethods]
impl PyTransactionReceipt {
    #[getter]
    pub fn status(&self) -> PyResult<u8> {
        Ok(self.inner.status as u8)
    }

    #[getter]
    pub fn account_id(&self) -> PyResult<Option<String>> {
        Ok(self.inner.account_id.as_ref().map(|id| id.to_string()))
    }

    #[getter]
    pub fn contract_id(&self) -> PyResult<Option<String>> {
        Ok(self.inner.contract_id.as_ref().map(|id| id.to_string()))
    }

    #[getter]
    pub fn file_id(&self) -> PyResult<Option<String>> {
        Ok(self.inner.file_id.as_ref().map(|id| id.to_string()))
    }
}
