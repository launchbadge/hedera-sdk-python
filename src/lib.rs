#![feature(specialization, type_ascription, transpose_result)]
#![recursion_limit = "1024"]
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
mod contract_function_result;
mod contract_info;
mod contract_log_info;
mod crypto;
mod duration;
mod either;
mod errors;
mod file_info;
mod id;
mod query_contract_get_bytecode;
mod query_contract_get_info;
mod query_contract_get_records;
mod query_crypto_get_account_balance;
mod query_crypto_get_account_records;
mod query_crypto_get_claim;
mod query_crypto_get_info;
mod query_file_get_contents;
mod query_file_get_info;
mod query_get_by_key;
mod query_transaction_get_receipt;
mod query_transaction_get_record;
mod timestamp;
mod transaction_admin_contract_recover;
mod transaction_admin_file_recover;
mod transaction_crypto_create;
mod transaction_id;
mod transaction_receipt;
mod transaction_record;
mod transaction_contract_call;

use self::{
    account_info::PyAccountInfo,
    claim::PyClaim,
    client::*,
    contract_function_result::PyContractFunctionResult,
    contract_info::PyContractInfo,
    contract_log_info::PyContractLogInfo,
    crypto::{PyPublicKey, PySecretKey, PySignature},
    duration::PyDuration,
    file_info::PyFileInfo,
    id::{PyAccountId, PyContractId, PyFileId},
    query_crypto_get_account_balance::PyQueryCryptoGetAccountBalance,
    query_crypto_get_account_records::PyQueryCryptoGetAccountRecords,
    query_crypto_get_claim::PyQueryCryptoGetClaim,
    query_file_get_info::PyQueryFileGetInfo,
    query_get_by_key::PyQueryGetByKey,
    query_transaction_get_receipt::PyQueryGetTransactionReceipt,
    query_transaction_get_record::PyQueryTransactionGetRecord,
    timestamp::PyDateTime,
    transaction_admin_contract_recover::PyTransactionAdminContractRecover,
    transaction_admin_file_recover::PyTransactionAdminFileRecover,
    transaction_crypto_create::PyTransactionCryptoCreate,
    transaction_id::PyTransactionId,
    transaction_receipt::PyTransactionReceipt,
    transaction_record::PyTransactionRecord,
    transaction_contract_call::PyTransactionContractCall,
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
    m.add_class::<PyQueryCryptoGetAccountRecords>()?;
    m.add_class::<PyQueryCryptoGetClaim>()?;
    m.add_class::<PyQueryTransactionGetRecord>()?;
    m.add_class::<PyQueryGetByKey>()?;
    m.add_class::<PyTransactionAdminFileRecover>()?;
    m.add_class::<PyTransactionAdminContractRecover>()?;
    m.add_class::<PyTransactionContractCall>()?;

    Ok(())
}
