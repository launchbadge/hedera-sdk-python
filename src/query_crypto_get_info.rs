use crate::PyAccountInfo;
use hedera::{query::QueryCryptoGetInfo, AccountId};

def_query!(QueryCryptoGetInfo(AccountId) -> PyAccountInfo);
