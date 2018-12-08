use super::errors::PyException;
use crate::contract_info::PyContractInfo;
use hedera::{
    query::{Query, QueryContractGetInfo},
    Client, ContractId, ContractInfo,
};
use pyo3::prelude::*;

#[pyclass(name = QueryContractGetInfo)]
pub struct PyQueryContractGetInfo {
    pub(crate) inner: Query<ContractInfo>,
}

impl PyQueryContractGetInfo {
    pub(crate) fn new(client: &Client, contract: ContractId) -> Self {
        Self {
            inner: QueryContractGetInfo::new(client, contract),
        }
    }
}

#[pymethods]
impl PyQueryContractGetInfo {
    pub fn get(&mut self) -> PyResult<PyContractInfo> {
        self.inner
            .get()
            .map(|info| PyContractInfo { inner: info })
            .map_err(PyException)
    }

    pub fn cost(&mut self) -> PyResult<u64> {
        self.inner.cost().map_err(PyException)
    }
}
