use super::errors::PyException;
use hedera::PublicKey;
use pyo3::prelude::*;

#[pyclass(name = PublicKey)]
pub struct PyPublicKey {
    inner: PublicKey,
}

#[pymethods]
impl PyPublicKey {
    #[new]
    pub fn __new__(obj: &PyRawObject, bytes: &str) -> PyResult<()> {
        let key = PublicKey::from_bytes(bytes).map_err(PyException)?;
        obj.init(|_| Self { inner: key })
    }
}

def_str!(PyPublicKey);
