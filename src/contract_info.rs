use crate::{PyAccountId, PyTimestamp, PyDuration, PyPublicKey};
use derive_more::From;
use hedera::ContractInfo;
use pyo3::prelude::*;
use try_from::TryInto;

#[pyclass(name = ContractInfo)]
#[derive(From)]
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

    #[getter]
    fn get_expiration_time(&self) -> PyResult<PyTimestamp> {
        self.inner.expiration_time.try_into()
    }

    #[getter]
    fn get_auto_renew_period(&self) -> PyResult<PyDuration> {
        self.inner.auto_renew_period.try_into()
    }

    #[getter]
    pub fn storage(&self) -> PyResult<i64> {
        Ok(self.inner.storage)
    }
}
