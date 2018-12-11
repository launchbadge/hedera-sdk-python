use derive_more::From;
// fixme: use hedera::ContractFunctionResult;
use crate::{PyContractId, PyContractLogInfo};
use hedera::query::ContractFunctionResult;
use itertools::Itertools;
use pyo3::prelude::*;

#[pyclass(name = ContractFunctionResult)]
#[derive(From)]
pub struct PyContractFunctionResult {
    pub(crate) inner: ContractFunctionResult,
}

#[pymethods]
impl PyContractFunctionResult {
    #[getter]
    pub fn contract_id(&self) -> PyResult<PyContractId> {
        Ok(self.inner.contract_id.into())
    }

    #[getter]
    pub fn contract_call_result(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.contract_call_result.clone())
    }

    #[getter]
    pub fn error_message(&self) -> PyResult<String> {
        Ok(self.inner.error_message.clone())
    }

    #[getter]
    pub fn bloom(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.bloom.clone())
    }

    #[getter]
    pub fn gas_used(&self) -> PyResult<u64> {
        Ok(self.inner.gas_used)
    }

    #[getter]
    pub fn log_info(&self) -> PyResult<Vec<PyContractLogInfo>> {
        Ok(self.inner.log_info.clone().into_iter().map_into().collect())
    }
}
