use hedera::{transaction::TransactionCryptoDeleteClaim, AccountId};

def_transaction!(TransactionCryptoDeleteClaim(AccountId, Vec<u8>){});
