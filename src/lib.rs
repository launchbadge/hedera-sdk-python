#![feature(specialization)]
#![warn(clippy::pedantic)]
#![allow(clippy::stutter)]
// todo: file an issue in pyo3 about this
#![allow(clippy::cast_ptr_alignment)]

#[macro_use]
mod macros;

mod client;
mod crypto;
mod errors;
mod id;
mod query_contract_get_bytecode;
mod query_crypto_get_account_balance;
mod query_file_get_contents;
mod query_file_get_info;
mod query_get_transaction_receipt;
mod timestamp;
mod transaction_id;
mod transaction_receipt;

use self::{client::*, query_crypto_get_account_balance::*, query_get_transaction_receipt::*};
use pyo3::prelude::*;

#[pymodinit]
fn hedera(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyClient>()?;
    m.add_class::<PyPartialAccountMessage>()?;
    m.add_class::<PyPartialFileMessage>()?;
    m.add_class::<PyPartialTransactionMessage>()?;
    m.add_class::<PyQueryCryptoGetAccountBalance>()?;
    m.add_class::<PyQueryGetTransactionReceipt>()?;
    m.add_class::<id::PyAccountId>()?;
    m.add_class::<id::PyFileId>()?;
    m.add_class::<id::PyContractId>()?;

    Ok(())
}
