use super::errors::PyValueError;
use derive_more::{From, Into};
use hedera::{PublicKey, SecretKey, Signature};
use pyo3::prelude::*;

#[pyclass(name = PublicKey)]
#[derive(From, Into, Clone)]
pub struct PyPublicKey {
    pub(crate) inner: PublicKey,
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
#[derive(From, Clone)]
pub struct PySecretKey {
    pub(crate) inner: SecretKey,
}

def_str!(PySecretKey);

#[pyclass(name = Signature)]
#[derive(From)]
pub struct PySignature {
    pub(crate) inner: Signature,
}

def_str!(PySignature);
