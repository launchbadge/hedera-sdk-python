use hedera::{query::QueryCryptoGetAccountBalance, AccountId};

def_query!(QueryCryptoGetAccountBalance(AccountId) -> u64);
