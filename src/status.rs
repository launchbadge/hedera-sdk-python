use hedera::Status;
use std::mem::transmute;
use derive_more::From;
use pyo3::{prelude::*};

#[pyclass(name = Status)]
#[derive(From)]
pub struct PyStatus {
    pub(crate) inner: Status,
}

#[pymethods]
impl PyStatus {
    #[new]
    pub fn __new__(obj: &PyRawObject, int: u8) -> PyResult<()> {
        let status: Status = unsafe { transmute(int) };
        obj.init(move || Self {
            inner: status
        })
    }

    pub fn get_int(&self) -> PyResult<u8> {
        Ok(self.inner as u8)
    }

    pub fn success(&self) -> PyResult<bool> {
        Ok(self.inner == Status::Success)
    }

    pub fn to_string(&self) -> PyResult<String> {
        Ok(format!("{:#?}", self.inner))
    }
}
