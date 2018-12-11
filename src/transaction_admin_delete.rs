use hedera::{
    transaction::{Transaction, TransactionAdminContractDelete, TransactionAdminFileDelete},
    Client, ContractId, FileId,
};
use pyo3::prelude::*;

#[pyclass(name = TransactionAdminFileDelete)]
pub struct PyTransactionAdminFileDelete {
    pub(crate) inner: Transaction<TransactionAdminFileDelete>,
}

impl PyTransactionAdminFileDelete {
    pub fn new(client: &Client, file: FileId) -> Self {
        Self {
            inner: TransactionAdminFileDelete::new(client, file),
        }
    }
}

//#[pymethods]
//impl PyTransactionAdminFileDelete {
//    pub fn expire_at(&mut self, time: DateTime<Utc>) -> &mut Self {
//        self.inner.expire_at(time);
//        self
//    }
//}

#[pyclass(name = TransactionAdminFileDelete)]
pub struct PyTransactionAdminContractDelete {
    pub(crate) inner: Transaction<TransactionAdminContractDelete>,
}

impl PyTransactionAdminContractDelete {
    pub fn new(client: &Client, contract: ContractId) -> Self {
        Self {
            inner: TransactionAdminContractDelete::new(client, contract),
        }
    }
}

//#[pymethods]
//impl PyTransactionAdminContractDelete {
//    pub fn expire_at(&mut self, time: DateTime<Utc>) -> &mut Self {
//        self.inner.expire_at(time);
//        self
//    }
//}
