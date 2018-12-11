use crate::{PyAccountId, PyClaim, PyDateTime, PyDuration, PyPublicKey};
use derive_more::From;
use hedera::AccountInfo;
use itertools::Itertools;
use pyo3::prelude::*;
use try_from::TryInto;

#[pyclass(name = AccountInfo)]
#[derive(From)]
pub struct PyAccountInfo {
    pub(crate) inner: AccountInfo,
}

#[pymethods]
impl PyAccountInfo {
    #[getter]
    pub fn account_id(&self) -> PyResult<String> {
        Ok(self.inner.account_id.to_string())
    }

    #[getter]
    pub fn contract_account_id(&self) -> PyResult<String> {
        Ok(self.inner.contract_account_id.to_string())
    }

    #[getter]
    pub fn deleted(&self) -> PyResult<bool> {
        Ok(self.inner.deleted as bool)
    }

    #[getter]
    pub fn proxy_account_id(&self) -> PyResult<Option<PyAccountId>> {
        Ok(self.inner.proxy_account_id.map(Into::into))
    }

    #[getter]
    pub fn proxy_fraction(&self) -> PyResult<i32> {
        Ok(self.inner.proxy_fraction as i32)
    }

    #[getter]
    pub fn proxy_received(&self) -> PyResult<i64> {
        Ok(self.inner.proxy_received as i64)
    }

    #[getter]
    pub fn key(&self) -> PyResult<PyPublicKey> {
        Ok(self.inner.key.clone().into())
    }

    #[getter]
    pub fn balance(&self) -> PyResult<u64> {
        Ok(self.inner.balance as u64)
    }

    #[getter]
    pub fn generate_send_record_threshold(&self) -> PyResult<u64> {
        Ok(self.inner.generate_send_record_threshold as u64)
    }

    #[getter]
    pub fn generate_receive_record_threshold(&self) -> PyResult<u64> {
        Ok(self.inner.generate_receive_record_threshold as u64)
    }

    #[getter]
    pub fn receiver_signature_required(&self) -> PyResult<bool> {
        Ok(self.inner.receiver_signature_required as bool)
    }

    #[getter]
    pub fn expiration_time(&self) -> PyResult<PyDateTime> {
        self.inner.expiration_time.try_into()
    }

    #[getter]
    pub fn auto_renew_period(&self) -> PyResult<PyDuration> {
        self.inner.auto_renew_period.try_into()
    }

    #[getter]
    pub fn claims(&self) -> PyResult<Vec<PyClaim>> {
        let claims = self.inner.claims.clone().into_iter().map_into().collect();

        Ok(claims)
    }
}
