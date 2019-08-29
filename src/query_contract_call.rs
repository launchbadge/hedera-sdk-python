use hedera::{query::QueryContractCall, ContractId};
use hedera::function_result::ContractFunctionResult;

def_query_new!(QueryContractCall: contract_call(ContractId, i64, array_of(u8), i64) -> QueryContractCall);

def_query_exec!(QueryContractCall: contract_call() -> ContractFunctionResult);
