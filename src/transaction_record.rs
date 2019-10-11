use crate::{id::PyAccountId, PyContractFunctionResult, PyTimestamp, PyTransactionReceipt};
use derive_more::From;
use hedera::{TransactionRecord, TransactionRecordBody};
use pyo3::prelude::*;
use try_from::TryInto;

// todo: add a .kind field to indicate what kind the body type is: ContractCall, ContractCreate, or Transfer

#[pyclass(name = TransactionRecord)]
#[derive(From)]
pub struct PyTransactionRecord {
    pub(crate) inner: TransactionRecord,
}

#[pymethods]
impl PyTransactionRecord {
    #[getter]
    pub fn receipt(&self) -> PyResult<PyTransactionReceipt> {
        Ok(self.inner.receipt.clone().into())
    }

    #[getter]
    pub fn transaction_hash(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.transaction_hash.clone())
    }

    #[getter]
    pub fn consensus_timestamp(&self) -> PyResult<Option<PyTimestamp>> {
        self.inner
            .consensus_timestamp
            .map(|ts| ts.try_into())
            .transpose()
    }

    #[getter]
    pub fn memo(&self) -> PyResult<String> {
        Ok(self.inner.memo.clone())
    }

    #[getter]
    pub fn transaction_fee(&self) -> PyResult<u64> {
        Ok(self.inner.transaction_fee)
    }

    #[getter]
    pub fn contract_function_result(&self) -> PyResult<Option<PyContractFunctionResult>> {
        match &self.inner.body {
            TransactionRecordBody::ContractCall(result) => Ok(Some(result.clone().into())),
            TransactionRecordBody::ContractCreate(result) => Ok(Some(result.clone().into())),

            _ => Ok(None),
        }
    }

    #[getter]
    pub fn transfers(&self) -> PyResult<Vec<(PyAccountId, i64)>> {
        match &self.inner.body {
            TransactionRecordBody::Transfer(transfers) => Ok(transfers
                .clone()
                .into_iter()
                .map(|(id, amount)| (id.into(), amount))
                .collect()),

            _ => Ok(vec![]),
        }
    }
}
