use super::{errors::PyException, transaction_receipt::PyTransactionReceipt, timestamp::py_date_time};
use hedera::{
    query::{Query, QueryCryptoGetAccountRecords},
    TransactionRecord,
    TransactionRecordBody,
    Client, AccountId
};
use pyo3::prelude::*;
use pyo3::types::PyDateTime;

#[pyclass(name = QueryCryptoGetAccountRecords)]
pub struct PyQueryCryptoGetAccountRecords {
    inner: Query<Vec<TransactionRecord>>
}

impl PyQueryCryptoGetAccountRecords {
    pub(crate) fn new(client: &Client, account_id: AccountId) -> Self {
        Self {
            inner: QueryCryptoGetAccountRecords::new(client, account_id)
        }
    }
}

#[pymethods]
impl PyQueryCryptoGetAccountRecords {
    pub fn get(&mut self) -> PyResult<Vec<PyTransactionRecord>>  {
        self.inner.get().map(|records| {
            records.into_iter().map(|record| {
                PyTransactionRecord {
                    inner: record
                }
            }).collect()
        }).map_err(PyException)
    }

    pub fn cost(&mut self) -> PyResult<u64> {
        self.inner.cost().map_err(PyException)
    }
}

#[pyclass(name = TranscationRecord)]
pub struct PyTransactionRecord {
    inner: TransactionRecord
}

#[pyclass(name = TranscationRecordBody)]
pub struct PyTransactionRecordBody {
    inner: TransactionRecordBody
}


#[pymethods]
impl PyTransactionRecord {
    #[getter]
    pub fn receipt(&self) -> PyResult<PyTransactionReceipt> {
        Ok(PyTransactionReceipt { inner: self.inner.receipt.clone()})
    }

    #[getter]
    pub fn transaction_hash(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.transaction_hash.clone())
    }

    pub fn consensus_timestamp(&self, py: Python) -> PyResult<Option<Py<PyDateTime>>> {
        match self.inner.consensus_timestamp {
            Some(datetime) => {
                Ok(Some(py_date_time(datetime, py)?))
            }
            None => Ok(None)
        }
    }

    #[getter]
    pub fn memo(&self) -> PyResult<String> {
        Ok(self.inner.memo.clone())
    }

    #[getter]
    pub fn transaction_fee(&self) -> PyResult<u64> {
        Ok(self.inner.transaction_fee.clone())
    }

    #[getter]
    pub fn body(&self) -> PyResult<PyTransactionRecordBody> {
        Ok(PyTransactionRecordBody {
            inner: self.inner.body.clone()
        } )
    }
}
