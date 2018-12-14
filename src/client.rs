use super::{
    errors::PyValueError, query_crypto_get_account_balance::*, query_crypto_get_info::*,
    query_file_get_contents::*, query_transaction_get_receipt::*,
};
use crate::{
    either::Either,
    id::{PyAccountId, PyContractId, PyFileId},
    transaction_id::PyTransactionId,
    PyQueryCryptoGetClaim, PyQueryFileGetInfo, PyQueryTransactionGetRecord,
    PyTransactionContractCall, PyTransactionContractCreate, PyTransactionContractUpdate,
    PyTransactionCryptoCreate, PyTransactionCryptoDelete, PyTransactionCryptoDeleteClaim,
    PyTransactionCryptoTransfer, PyTransactionCryptoUpdate, PyTransactionFileAppend,
    PyTransactionFileCreate, PyTransactionFileDelete,
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

    pub fn transfer_crypto(&self) -> PyResult<PyTransactionCryptoTransfer> {
        Ok(PyTransactionCryptoTransfer::new(&self.inner))
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

    pub fn create_contract(&self) -> PyResult<PyTransactionContractCreate> {
        Ok(PyTransactionContractCreate::new(&self.inner))
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

    pub fn create_file(&self) -> PyResult<PyTransactionFileCreate> {
        Ok(PyTransactionFileCreate::new(&self.inner))
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
    pub fn update(&self) -> PyResult<PyTransactionCryptoUpdate> {
        Ok(PyTransactionCryptoUpdate::new(&self.client, self.account))
    }

    pub fn delete(&self) -> PyResult<PyTransactionCryptoDelete> {
        Ok(PyTransactionCryptoDelete::new(&self.client, self.account))
    }

    pub fn claim(&self, hash: Vec<u8>) -> PyResult<PyPartialAccountClaimMessage> {
        Ok(PyPartialAccountClaimMessage {
            client: Rc::clone(&self.client),
            account: self.account,
            hash,
        })
    }
}

#[pyclass(name = PartialAccountClaimMessage)]
pub struct PyPartialAccountClaimMessage {
    client: Rc<Client>,
    account: AccountId,
    hash: Vec<u8>,
}

#[pymethods]
impl PyPartialAccountClaimMessage {
    /// receipt(self) -> QueryGetTransactionReceipt
    /// --
    ///
    /// Get the receipt of the transaction.
    ///
    /// Once a transaction reaches consensus, then information about whether it succeeded or
    /// failed will be available until the end of the receipt period (180 seconds).
    pub fn delete(&self) -> PyResult<PyTransactionCryptoDeleteClaim> {
        Ok(PyTransactionCryptoDeleteClaim::new(
            &self.client,
            self.account,
            self.hash.clone(),
        ))
    }
    pub fn get(&self) -> PyResult<PyQueryCryptoGetClaim> {
        Ok(PyQueryCryptoGetClaim::new(
            &self.client,
            self.account,
            self.hash.clone(),
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
    pub fn append(&self, contents: Vec<u8>) -> PyResult<PyTransactionFileAppend> {
        Ok(PyTransactionFileAppend::new(
            &self.client,
            self.file,
            contents,
        ))
    }
    pub fn delete(&self) -> PyResult<PyTransactionFileDelete> {
        Ok(PyTransactionFileDelete::new(&self.client, self.file))
    }
    pub fn info(&self) -> PyResult<PyQueryFileGetInfo> {
        Ok(PyQueryFileGetInfo::new(&self.client, self.file))
    }
    pub fn contents(&self) -> PyResult<PyQueryFileGetContents> {
        Ok(PyQueryFileGetContents::new(&self.client, self.file))
    }
}

#[pyclass(name = PartialContractMessage)]
pub struct PyPartialContractMessage {
    client: Rc<Client>,
    contract: ContractId,
}

#[pymethods]
impl PyPartialContractMessage {
    pub fn call(&self) -> PyResult<PyTransactionContractCall> {
        Ok(PyTransactionContractCall::new(&self.client, self.contract))
    }

#[pyclass(name = PartialFileMessage)]
pub struct PyPartialFileMessage {
    pub fn update(&self) -> PyResult<PyTransactionContractUpdate> {
        Ok(PyTransactionContractUpdate::new(
            &self.client,
            self.contract,
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
    pub fn contents(&self) -> PyResult<PyQueryFileGetContents> {
        Ok(PyQueryFileGetContents::new(&self.client, self.file))
    pub fn receipt(&self) -> PyResult<PyQueryTransactionGetReceipt> {
        Ok(PyQueryTransactionGetReceipt::new(
            &self.client,
            self.transaction.clone(),
        ))
    }
    pub fn record(&self) -> PyResult<PyQueryTransactionGetRecord> {
        Ok(PyQueryTransactionGetRecord::new(
            &self.client,
            self.transaction.clone(),
        ))
    }
}
