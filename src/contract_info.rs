use crate::{crypto::PyPublicKey, id::PyAccountId, timestamp::py_date_time};
use hedera::ContractInfo;
use pyo3::{
    prelude::*,
    types::{PyDateTime, PyDelta},
};

#[pyclass(name = ContractInfo)]
pub struct PyContractInfo {
    pub(crate) inner: ContractInfo,
}

#[pymethods]
impl PyContractInfo {
    #[getter]
    pub fn account_id(&self) -> PyResult<PyAccountId> {
        Ok(PyAccountId {
            inner: self.inner.account_id,
        })
    }

    #[getter]
    pub fn contract_account_id(&self) -> PyResult<String> {
        Ok(self.inner.contract_account_id.clone())
    }

    #[getter]
    pub fn admin_key(&self) -> PyResult<Option<PyPublicKey>> {
        Ok(self
            .inner
            .admin_key
            .clone()
            .map(|key| PyPublicKey { inner: key }))
    }

    pub fn expiration_time(&self, py: Python) -> PyResult<Py<PyDateTime>> {
        py_date_time(self.inner.expiration_time, py)
    }

    pub fn auto_renew_period(&self, py: Python) -> PyResult<Py<PyDelta>> {
        let renew_period = self.inner.auto_renew_period;
        let seconds = renew_period.as_secs() as i32;
        let microseconds = renew_period.subsec_micros() as i32;

        PyDelta::new(py, 0, seconds, microseconds, false)
    }

    #[getter]
    pub fn storage(&self) -> PyResult<i64> {
        Ok(self.inner.storage)
    }
}
