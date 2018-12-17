use crate::{crypto::PyPublicKey, id::PyAccountId};
use derive_more::{From, Into};
use hedera::Claim;
use itertools::Itertools;
use pyo3::prelude::*;

#[pyclass(name = Claim)]
#[derive(From, Into, Clone)]
pub struct PyClaim {
    inner: Claim,
}

#[pymethods]
impl PyClaim {
    #[getter]
    pub fn account(&self) -> PyResult<PyAccountId> {
        Ok(self.inner.account.into())
    }

    #[getter]
    pub fn hash(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.hash.clone())
    }

    #[getter]
    pub fn keys(&self) -> PyResult<Vec<PyPublicKey>> {
        let keys = self.inner.keys.clone().into_iter().map_into().collect();

        Ok(keys)
    }
}
