use hedera::{query::QueryContractCall, ContractId};
use crate::contract_function_result::PyContractFunctionResult;

//def_query_new!(QueryContractCall: contract_call(ContractId, i64, array_of(u8), i64) -> QueryContractCall);
//
//def_query_exec!(QueryContractCall: contract_call() -> ContractFunctionResult);

def_query!(QueryContractCall(ContractId, i64, Vec<u8>, i64) -> PyContractFunctionResult);
