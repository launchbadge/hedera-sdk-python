use super::errors::PyValueError;
use derive_more::{From, Into};
use hedera::{AccountId, ContractId, FileId};
use pyo3::prelude::*;
use std::str::FromStr;

macro_rules! impl_id {
    ($pyname:ident, $rname:ident) => {
        #[pymethods]
        impl $pyname {
            #[new]
            fn __new__(obj: &PyRawObject, s: &str) -> PyResult<()> {
                let id = $rname::from_str(s).map_err(PyValueError)?;
                obj.init(|| Self { inner: id })
            }
        }

        def_str!($pyname);
    };
}

#[pyclass(name = AccountId)]
#[derive(Clone, From, Into)]
pub struct PyAccountId {
    pub(crate) inner: AccountId,
}

impl_id!(PyAccountId, AccountId);

#[pyclass(name = FileId)]
#[derive(Clone, From, Into)]
pub struct PyFileId {
    pub(crate) inner: FileId,
}

impl_id!(PyFileId, FileId);

#[pyclass(name = ContractId)]
#[derive(Clone, From, Into)]
pub struct PyContractId {
    pub(crate) inner: ContractId,
}

impl_id!(PyContractId, ContractId);
