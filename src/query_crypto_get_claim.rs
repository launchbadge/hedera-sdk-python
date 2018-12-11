use crate::PyClaim;
use hedera::{query::QueryCryptoGetClaim, AccountId};

def_query!(QueryCryptoGetClaim(AccountId, Vec<u8>) -> PyClaim);
