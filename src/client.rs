use super::{
    errors::PyValueError, query_crypto_get_account_balance::*, query_crypto_get_info::*,
    query_file_get_contents::*, query_transaction_get_receipt::*,
};
use crate::{
    either::Either,
    id::{PyAccountId, PyContractId, PyFileId},
    transaction_id::PyTransactionId,
    PyTransactionCryptoCreate,
};
use hedera::{AccountId, Client, ContractId, FileId, TransactionId};
use pyo3::{prelude::*, types::PyObjectRef};
use std::rc::Rc;
use try_from::TryInto;

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

    pub fn create_account(&self) -> PyResult<PyTransactionCryptoCreate> {
        Ok(PyTransactionCryptoCreate::new(&self.inner))
    }

    /// account(self, id: str) -> PartialAccountMessage
    /// --
    ///
    /// Access available operations on a single crypto-currency account.
    pub fn account(&self, id: &PyObjectRef) -> PyResult<PyPartialAccountMessage> {
        Ok(PyPartialAccountMessage {
            client: Rc::clone(&self.inner),
            account: (FromPyObject::extract(id)?: Either<&str, &PyAccountId>).try_into()?,
        })
    }

    /// contract(self, id: str) -> PartialContractMessage
    /// --
    ///
    /// Access available operations on a single smart contract.
    pub fn contract(&self, id: &PyObjectRef) -> PyResult<PyPartialContractMessage> {
        Ok(PyPartialContractMessage {
            client: Rc::clone(&self.inner),
            contract: (FromPyObject::extract(id)?: Either<&str, &PyContractId>).try_into()?,
        })
    }

    /// transaction(self, id: str) -> PartialTransactionMessage
    /// --
    ///
    /// Access available operations on a single transaction.
    pub fn transaction(&self, id: &PyObjectRef) -> PyResult<PyPartialTransactionMessage> {
        Ok(PyPartialTransactionMessage {
            client: Rc::clone(&self.inner),
            transaction: (FromPyObject::extract(id)?: Either<&str, &PyTransactionId>).try_into()?,
        })
    }

    /// file(self, id: str) -> PartialFileMessage
    /// --
    ///
    /// Access available operations on a single file.
    pub fn file(&self, id: &PyObjectRef) -> PyResult<PyPartialFileMessage> {
        Ok(PyPartialFileMessage {
            client: Rc::clone(&self.inner),
            file: (FromPyObject::extract(id)?: Either<&str, &PyFileId>).try_into()?,
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

    pub fn info(&self) -> PyResult<PyQueryCryptoGetInfo> {
        Ok(PyQueryCryptoGetInfo::new(&self.client, self.account))
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

#[pyclass(name = PartialContractMessage)]
pub struct PyPartialContractMessage {
    client: Rc<Client>,
    contract: ContractId,
}

#[pymethods]
impl PyPartialContractMessage {}

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
