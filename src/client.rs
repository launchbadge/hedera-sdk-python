use super::{
    errors::PyValueError, query_crypto_get_account_balance::*, query_file_get_contents::*,
    query_get_transaction_receipt::*,
};
use crate::{
    id::{PyAccountId, PyFileId},
    transaction_id::PyTransactionId,
};
use hedera::{AccountId, Client, FileId, TransactionId};
use pyo3::prelude::*;
use std::rc::Rc;

#[pyclass(name = Client)]
pub struct PyClient {
    inner: Rc<Client>,
}

#[pymethods]
impl PyClient {
    #[new]
    pub fn __new__(obj: &PyRawObject, address: &str) -> PyResult<()> {
        let client = Client::new(address).map_err(PyValueError)?;
        obj.init(move |_| Self {
            inner: Rc::new(client),
        })
    }

    /// account(self, id: str) -> PartialAccountMessage
    /// --
    ///
    /// Access available operations on a single crypto-currency account.
    pub fn account(&self, id: &PyAccountId) -> PyResult<PyPartialAccountMessage> {
        Ok(PyPartialAccountMessage {
            client: Rc::clone(&self.inner),
            account: id.inner,
        })
    }

    /// transaction(self, id: str) -> PartialTransactionMessage
    /// --
    ///
    /// Access available operations on a single transaction.
    pub fn transaction(&self, id: &PyTransactionId) -> PyResult<PyPartialTransactionMessage> {
        Ok(PyPartialTransactionMessage {
            client: Rc::clone(&self.inner),
            transaction: id.inner.clone(),
        })
    }

    /// file(self, id: str) -> PartialFileMessage
    /// --
    ///
    /// Access available operations on a single file.
    pub fn file(&self, id: &PyFileId) -> PyResult<PyPartialFileMessage> {
        Ok(PyPartialFileMessage {
            client: Rc::clone(&self.inner),
            file: id.inner,
        })
    }
}

#[pyclass(name = PartialAccountMessage)]
pub struct PyPartialAccountMessage {
    client: Rc<Client>,
    account: AccountId,
}

#[pymethods]
impl PyPartialAccountMessage {
    /// balance(self) -> QueryCryptoGetAccountBalance
    /// --
    ///
    /// Get the balance of a crypto-currency account.
    ///
    /// This returns only the balance, so it is a smaller and faster reply than
    /// :py:method:`hedera.PartialAccountMessage.info`.
    pub fn balance(&self) -> PyResult<PyQueryCryptoGetAccountBalance> {
        Ok(PyQueryCryptoGetAccountBalance::new(
            &self.client,
            self.account,
        ))
    }
}

#[pyclass(name = PartialTransactionMessage)]
pub struct PyPartialTransactionMessage {
    client: Rc<Client>,
    transaction: TransactionId,
}

#[pymethods]
impl PyPartialTransactionMessage {
    /// receipt(self) -> QueryGetTransactionReceipt
    /// --
    ///
    /// Get the receipt of the transaction.
    ///
    /// Once a transaction reaches consensus, then information about whether it succeeded or
    /// failed will be available until the end of the receipt period (180 seconds).
    pub fn receipt(&self) -> PyResult<PyQueryGetTransactionReceipt> {
        Ok(PyQueryGetTransactionReceipt::new(
            &self.client,
            self.transaction.clone(),
        ))
    }
}

#[pyclass(name = PartialFileMessage)]
pub struct PyPartialFileMessage {
    client: Rc<Client>,
    file: FileId,
}

#[pymethods]
impl PyPartialFileMessage {
    pub fn contents(&self) -> PyResult<PyQueryFileGetContents> {
        Ok(PyQueryFileGetContents::new(&self.client, self.file))
    }
}
