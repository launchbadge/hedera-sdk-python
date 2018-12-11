use crate::PyContractInfo;
use hedera::{query::QueryContractGetInfo, ContractId};

def_query!(QueryContractGetInfo(ContractId) -> PyContractInfo);
