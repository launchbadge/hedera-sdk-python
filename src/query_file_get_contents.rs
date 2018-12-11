use hedera::{query::QueryFileGetContents, FileId};

def_query!(QueryFileGetContents(FileId) -> Vec<u8>);
