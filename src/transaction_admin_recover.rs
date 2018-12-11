use hedera::{
    transaction::{Transaction, TransactionAdminContractRecover, TransactionAdminFileRecover},
    Client, ContractId, FileId,
};
use pyo3::prelude::*;

#[pyclass(name = TransactionAdminFileRecover)]
pub struct PyTransactionAdminFileRecover {
    pub inner: Transaction<TransactionAdminFileRecover>,
}

impl PyTransactionAdminFileRecover {
    pub fn new(client: &Client, file: FileId) -> Self {
        Self {
            inner: TransactionAdminFileRecover::new(client, file),
        }
    }
}

#[pyclass(name = TransactionAdminContractRecover)]
pub struct PyTransactionAdminContractRecover {
    pub inner: Transaction<TransactionAdminContractRecover>,
}

impl PyTransactionAdminContractRecover {
    pub fn new(client: &Client, contract: ContractId) -> Self {
        Self {
            inner: TransactionAdminContractRecover::new(client, contract),
        }
    }
}
