#![feature(specialization, type_ascription)]
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
mod call_params;
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
mod solidity_util;
mod timestamp;
mod transaction_contract_call;
mod transaction_contract_create;
mod transaction_contract_update;
mod transaction_contract_delete;
mod transaction_crypto_add_claim;
mod transaction_crypto_create;
mod transaction_crypto_delete;
mod transaction_crypto_delete_claim;
mod transaction_crypto_transfer;
mod transaction_crypto_update;
mod transaction_file_append;
mod transaction_file_create;
mod transaction_file_delete;
mod transaction_file_update;
mod transaction_id;
mod transaction_receipt;
mod transaction_record;

use self::{
    account_info::PyAccountInfo,
    claim::PyClaim,
    client::*,
    call_params::PyCallParams,
    contract_function_result::PyContractFunctionResult,
    contract_info::PyContractInfo,
    contract_log_info::PyContractLogInfo,
    crypto::{PyPublicKey, PySecretKey, PySignature},
    duration::PyDuration,
    file_info::PyFileInfo,
    id::{PyAccountId, PyContractId, PyFileId},
    query_contract_get_bytecode::PyQueryContractGetBytecode,
    query_contract_get_info::PyQueryContractGetInfo,
    query_contract_get_records::PyQueryContractGetRecords,
    query_crypto_get_account_balance::PyQueryCryptoGetAccountBalance,
    query_crypto_get_account_records::PyQueryCryptoGetAccountRecords,
    query_crypto_get_claim::PyQueryCryptoGetClaim,
    query_crypto_get_info::PyQueryCryptoGetInfo,
    query_file_get_contents::PyQueryFileGetContents,
    query_file_get_info::PyQueryFileGetInfo,
    query_get_by_key::PyQueryGetByKey,
    query_transaction_get_receipt::PyQueryTransactionGetReceipt,
    query_transaction_get_record::PyQueryTransactionGetRecord,
    timestamp::PyDateTime,
    transaction_contract_call::PyTransactionContractCall,
    transaction_contract_create::PyTransactionContractCreate,
    transaction_contract_update::PyTransactionContractUpdate,
    transaction_contract_delete::PyTransactionContractDelete,
    transaction_crypto_add_claim::PyTransactionCryptoAddClaim,
    transaction_crypto_create::PyTransactionCryptoCreate,
    transaction_crypto_delete::PyTransactionCryptoDelete,
    transaction_crypto_delete_claim::PyTransactionCryptoDeleteClaim,
    transaction_crypto_transfer::PyTransactionCryptoTransfer,
    transaction_crypto_update::PyTransactionCryptoUpdate,
    transaction_file_append::PyTransactionFileAppend,
    transaction_file_create::PyTransactionFileCreate,
    transaction_file_delete::PyTransactionFileDelete,
    transaction_file_update::PyTransactionFileUpdate,
    transaction_id::PyTransactionId,
    transaction_receipt::PyTransactionReceipt,
    transaction_record::PyTransactionRecord,
};

use pyo3::prelude::*;
use pyo3::wrap_module;
use crate::solidity_util::PyInit_solidity_utils;

#[pymodule]
fn hedera(_py: Python, m: &PyModule) -> PyResult<()> {
    // Client-related types
    m.add_class::<PyClient>()?;
    m.add_class::<PyPartialAccountClaimMessage>()?;
    m.add_class::<PyPartialAccountMessage>()?;
    m.add_class::<PyPartialContractMessage>()?;
    m.add_class::<PyPartialFileMessage>()?;
    m.add_class::<PyPartialTransactionMessage>()?;

    // Common types
    m.add_class::<PyAccountId>()?;
    m.add_class::<PyAccountInfo>()?;
    m.add_class::<PyContractId>()?;
    m.add_class::<PyContractInfo>()?;
    m.add_class::<PyFileId>()?;
    m.add_class::<PyFileInfo>()?;
    m.add_class::<PyPublicKey>()?;
    m.add_class::<PySecretKey>()?;
    m.add_class::<PySignature>()?;
    m.add_class::<PyTransactionId>()?;
    m.add_class::<PyTransactionReceipt>()?;

    // Query types
    m.add_class::<PyQueryContractGetBytecode>()?;
    m.add_class::<PyQueryContractGetInfo>()?;
    m.add_class::<PyQueryContractGetRecords>()?;
    m.add_class::<PyQueryCryptoGetAccountBalance>()?;
    m.add_class::<PyQueryCryptoGetAccountRecords>()?;
    m.add_class::<PyQueryCryptoGetClaim>()?;
    m.add_class::<PyQueryCryptoGetInfo>()?;
    m.add_class::<PyQueryFileGetContents>()?;
    m.add_class::<PyQueryFileGetInfo>()?;
    m.add_class::<PyQueryGetByKey>()?;
    m.add_class::<PyQueryTransactionGetReceipt>()?;
    m.add_class::<PyQueryTransactionGetRecord>()?;

    // Transaction types
    m.add_class::<PyTransactionContractCall>()?;
    m.add_class::<PyTransactionContractCreate>()?;
    m.add_class::<PyTransactionContractUpdate>()?;
    m.add_class::<PyTransactionContractDelete>()?;
    m.add_class::<PyTransactionCryptoAddClaim>()?;
    m.add_class::<PyTransactionCryptoCreate>()?;
    m.add_class::<PyTransactionCryptoDelete>()?;
    m.add_class::<PyTransactionCryptoDeleteClaim>()?;
    m.add_class::<PyTransactionCryptoTransfer>()?;
    m.add_class::<PyTransactionCryptoUpdate>()?;
    m.add_class::<PyTransactionFileAppend>()?;
    m.add_class::<PyTransactionFileCreate>()?;
    m.add_class::<PyTransactionFileDelete>()?;
    m.add_class::<PyTransactionFileUpdate>()?;

    // Param types
    m.add_class::<PyCallParams>()?;

    // Sub Modules
    m.add_wrapped(wrap_module!(solidity_utils))?;

    Ok(())
}
