use hedera::solidity_util::*;
use pyo3::{prelude::*, types::PyObjectRef, wrap_function};
use try_from::TryInto;
use crate::id::{PyAccountId, PyContractId, PyFileId};
use crate::either::Either;

#[pyfunction]
pub fn address_from_account(acct_id: &PyObjectRef) -> PyResult<String> {
    let acct = (FromPyObject::extract(acct_id)?: Either<&str, &PyAccountId>).try_into()?;
    let addr = address_for_account(acct);
    Ok(addr)
}

#[pyfunction]
pub fn address_from_contract(contract_id: &PyObjectRef) -> PyResult<String> {
    let contract = (FromPyObject::extract(contract_id)?: Either<&str, &PyContractId>).try_into()?;
    let addr = address_for_contract(contract);
    Ok(addr)
}

#[pyfunction]
pub fn address_from_file(file_id: &PyObjectRef) -> PyResult<String> {
    let f = (FromPyObject::extract(file_id)?: Either<&str, &PyFileId>).try_into()?;
    let addr = address_for_file(f);
    Ok(addr)
}

#[pyfunction]
pub fn account_from_address(addr: String) -> PyResult<PyAccountId> {
    let acct = account_for_address(addr);
    let py_acct = PyAccountId{inner: acct};
    Ok(py_acct)
}

#[pyfunction]
pub fn contract_from_address(addr: String) -> PyResult<PyContractId> {
    let contract = account_for_contract(addr);
    let py_contract = PyContractId{inner: contract};
    Ok(py_contract)
}

#[pyfunction]
pub fn file_from_address(addr: String) -> PyResult<PyFileId> {
    let file = account_for_file(addr);
    let py_file = PyFileId{inner: file};
    Ok(py_file)
}

#[pymodule]
pub fn solidity_utils(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_function!(address_from_account))?;
    module.add_wrapped(wrap_function!(address_from_contract))?;
    module.add_wrapped(wrap_function!(address_from_file))?;
    module.add_wrapped(wrap_function!(account_from_address))?;
    module.add_wrapped(wrap_function!(contract_from_address))?;
    module.add_wrapped(wrap_function!(file_from_address))?;
    Ok(())
}
