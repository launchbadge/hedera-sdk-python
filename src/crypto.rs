use super::errors::PyValueError;
use hedera::{PublicKey, SecretKey};
use pyo3::prelude::*;

#[pyclass(name = PublicKey)]
pub struct PyPublicKey {
    inner: PublicKey,
}

#[pymethods]
impl PyPublicKey {
    #[new]
    pub fn __new__(obj: &PyRawObject, s: &str) -> PyResult<()> {
        let key = PublicKey::from_bytes(s).map_err(PyValueError)?;
        obj.init(|_| Self { inner: key })
    }
}

def_str!(PyPublicKey);

#[pyclass(name = SecretKey)]
pub struct PySecretKey {
    inner: SecretKey,
}

def_str!(PySecretKey);
