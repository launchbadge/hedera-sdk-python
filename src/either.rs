use pyo3::{types::PyObjectRef, FromPyObject, PyResult};
use try_from::TryFrom;
use failure::Error;

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
