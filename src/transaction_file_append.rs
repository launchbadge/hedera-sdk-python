use hedera::{transaction::TransactionFileAppend, FileId};

def_transaction!(TransactionFileAppend(FileId, Vec<u8>) {});
