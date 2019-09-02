use derive_more::From;
use crate::{PyContractId, PyContractLogInfo};
use hedera::function_result::ContractFunctionResult;
use itertools::Itertools;
use pyo3::{prelude::*, exceptions};

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

    pub fn get_int(&self, val_index: usize) -> PyResult<isize> {
        Ok(self.inner.get_int(val_index))
    }

    pub fn get_long(&self, val_index: usize) -> PyResult<i64> {
        Ok(self.inner.get_long(val_index))
    }

    pub fn get_bytes(&self, val_index: usize) -> PyResult<Vec<u8>> {
        Ok(self.inner.get_bytes(val_index))
    }

    pub fn get_byte_array(&self, val_index: usize) -> PyResult<Vec<Vec<u8>>> {
        Ok(self.inner.get_byte_array(val_index))
    }

    pub fn get_string(&self, val_index: usize) -> PyResult<String> {
        match self.inner.get_string(val_index){
            Ok(s) => Ok(s),
            Err(e) => Err(exceptions::Exception::py_err(format!("{:#?}", e)))
        }
    }

    pub fn get_bool(&self, val_index: usize) -> PyResult<bool> {
        Ok(self.inner.get_bool(val_index))
    }

    pub fn get_address(&self, val_index: usize) -> PyResult<Vec<u8>> {
        Ok(self.inner.get_address(val_index))
    }

    pub fn get_address_array(&self, val_index: usize) -> PyResult<Vec<String>> {
        Ok(self.inner.get_address_array(val_index))
    }
}
