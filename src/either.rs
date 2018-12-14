use crate::{
    id::{PyAccountId, PyContractId, PyFileId},
    transaction_id::PyTransactionId,
};
use hedera::{AccountId, ContractId, FileId, TransactionId};
use pyo3::{types::PyObjectRef, FromPyObject, PyResult};

#[derive(Debug)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<'a, A, B> FromPyObject<'a> for Either<A, B>
where
    A: FromPyObject<'a>,
    B: FromPyObject<'a>,
{
    fn extract(ob: &'a PyObjectRef) -> PyResult<Self> {
        A::extract(ob)
            .map(Either::Left)
            .or_else(|_| B::extract(ob).map(Either::Right))
    }
}

try_from_either!(PyAccountId, AccountId);
try_from_either!(PyFileId, FileId);
try_from_either!(PyContractId, ContractId);
try_from_either!(PyTransactionId, TransactionId);
