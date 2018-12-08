use super::errors::PyValueError;
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
                obj.init(|_| Self { inner: id })
            }
        }

        def_str!($pyname);
    };
}

// NOTE: this would be in the above macro if it weren't for the pyclass attribute
#[pyclass(name = AccountId)]
pub struct PyAccountId {
    pub(crate) inner: AccountId,
}

impl_id!(PyAccountId, AccountId);

#[pyclass(name = FileId)]
pub struct PyFileId {
    pub(crate) inner: FileId,
}

impl_id!(PyFileId, FileId);

#[pyclass(name = ContractId)]
pub struct PyContractId {
    pub(crate) inner: ContractId,
}

impl_id!(PyContractId, ContractId);
