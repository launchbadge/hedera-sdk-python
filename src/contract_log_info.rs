use derive_more::From;
// fixme: use hedera::ContractLogInfo;
use hedera::function_result::ContractLogInfo;
use pyo3::prelude::*;

#[pyclass(name = ContractLogInfo)]
#[derive(From)]
pub struct PyContractLogInfo {
    pub(crate) inner: ContractLogInfo,
}

#[pymethods]
impl PyContractLogInfo {
    #[getter]
    pub fn bloom(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.bloom.clone())
    }

    #[getter]
    pub fn topic(&self) -> PyResult<Vec<Vec<u8>>> {
        Ok(self.inner.topic.clone())
    }

    #[getter]
    pub fn data(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.data.clone())
    }
}
