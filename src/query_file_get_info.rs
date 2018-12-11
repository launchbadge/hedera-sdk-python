use crate::PyFileInfo;
use hedera::{query::QueryFileGetInfo, FileId};

def_query!(QueryFileGetInfo(FileId) -> PyFileInfo);
