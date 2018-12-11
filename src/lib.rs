#![feature(specialization)]
#![recursion_limit = "256"]
#![warn(clippy::pedantic)]
#![allow(clippy::stutter)]
// todo: file an issue in pyo3 about this
#![allow(clippy::cast_ptr_alignment)]

#[macro_use]
extern crate mashup;

#[macro_use]
mod macros;

mod account_info;
mod claim;
mod client;
mod contract_info;
mod crypto;
mod duration;
mod errors;
mod file_info;
mod id;
mod query_contract_get_bytecode;
mod query_contract_get_info;
mod query_crypto_get_account_balance;
mod query_crypto_get_info;
mod query_file_get_contents;
mod query_file_get_info;
mod query_get_transaction_receipt;
mod timestamp;
mod transaction_id;
mod transaction_receipt;

use self::{
    account_info::PyAccountInfo,
    claim::PyClaim,
    client::*,
    contract_info::PyContractInfo,
    crypto::{PyPublicKey, PySecretKey, PySignature},
    duration::PyDuration,
    file_info::PyFileInfo,
    id::{PyAccountId, PyContractId, PyFileId},
    query_crypto_get_account_balance::PyQueryCryptoGetAccountBalance,
    query_file_get_info::PyQueryFileGetInfo,
    query_get_transaction_receipt::PyQueryGetTransactionReceipt,
    timestamp::PyDateTime,
    transaction_id::PyTransactionId,
    transaction_receipt::PyTransactionReceipt,
};

use pyo3::prelude::*;

#[pymodinit]
fn hedera(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyClient>()?;
    m.add_class::<PyPartialAccountMessage>()?;
    m.add_class::<PyPartialFileMessage>()?;
    m.add_class::<PyPartialTransactionMessage>()?;
    m.add_class::<PyQueryCryptoGetAccountBalance>()?;
    m.add_class::<PyQueryGetTransactionReceipt>()?;
    m.add_class::<PyQueryFileGetInfo>()?;
    m.add_class::<PyAccountId>()?;
    m.add_class::<PyFileId>()?;
    m.add_class::<PyContractId>()?;
    m.add_class::<PyPublicKey>()?;
    m.add_class::<PySecretKey>()?;
    m.add_class::<PySignature>()?;
    m.add_class::<PyAccountInfo>()?;
    m.add_class::<PyContractInfo>()?;
    m.add_class::<PyFileInfo>()?;
    m.add_class::<PyTransactionId>()?;
    m.add_class::<PyTransactionReceipt>()?;

    Ok(())
}
