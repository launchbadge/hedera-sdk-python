use hedera::{query::QueryContractGetBytecode, ContractId};

def_query!(QueryContractGetBytecode(ContractId) -> Vec<u8>);
