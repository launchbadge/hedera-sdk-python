use super::{
    errors::PyValueError, query_crypto_get_account_balance::*, query_file_get_contents::*,
    query_get_transaction_receipt::*,
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
    /// __new__(address: str)
    /// --
    ///
    /// Establish a new connection to the Hedera API.
    #[new]
    pub fn __new__(obj: &PyRawObject, address: String) -> PyResult<()> {
        let client = Client::new(&address).map_err(PyValueError)?;
        obj.init(move |_| PyClient {
            inner: Rc::new(client),
        })
    }

    /// account(self, id: str) -> PartialAccountMessage
    /// --
    ///
    /// Access available operations on a single crypto-currency account.
    pub fn account(&self, id: String) -> PyResult<PyPartialAccountMessage> {
        Ok(PyPartialAccountMessage {
            client: Rc::clone(&self.inner),
            account: id.parse().map_err(PyValueError)?,
        })
    }

    /// transaction(self, id: str) -> PartialTransactionMessage
    /// --
    ///
    /// Access available operations on a single transaction.
    pub fn transaction(&self, id: String) -> PyResult<PyPartialTransactionMessage> {
        Ok(PyPartialTransactionMessage {
            client: Rc::clone(&self.inner),
            transaction: id.parse().map_err(PyValueError)?,
        })
    }

    /// file(self, id: str) -> PartialFileMessage
    /// --
    ///
    /// Access available operations on a single file.
    pub fn file(&self, id: String) -> PyResult<PyPartialFileMessage> {
        Ok(PyPartialFileMessage {
            client: Rc::clone(&self.inner),
            file: id.parse().map_err(PyValueError)?,
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
        Ok(PyQueryFileGetContents::new(&self.client, self.file.clone()))
    }
}
