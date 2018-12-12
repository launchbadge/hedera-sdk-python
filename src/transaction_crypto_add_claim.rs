use hedera::{transaction::TransactionCryptoAddClaim, AccountId, Claim};

def_transaction!(TransactionCryptoAddClaim(AccountId, Claim){});
