use hedera::{query::QueryContractCall, ContractId};
use crate::contract_function_result::PyContractFunctionResult;

def_query!(QueryContractCall(ContractId, i64, Vec<u8>, i64) -> PyContractFunctionResult);
