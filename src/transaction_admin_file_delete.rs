use hedera::{transaction::TransactionAdminFileDelete, FileId};

def_transaction!(TransactionAdminFileDelete(FileId) {});
