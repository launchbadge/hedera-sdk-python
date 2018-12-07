use hedera::{
    query::{Query, QueryFileGetInfo, QueryFileGetInfoResponse},
    Client, FileId,
};
use super::timestamp::py_date_time;
use pyo3::prelude::*;
use pyo3::types::PyDateTime;
use crate::errors::PyException;

#[pyclass(name = QueryFileGetInfo)]
pub struct PyQueryFileGetInfo {
    inner: Query<QueryFileGetInfoResponse>,
}

impl PyQueryFileGetInfo {
    pub(crate) fn new(client: &Client, file: FileId) -> Self {
        Self {
            inner: QueryFileGetInfo::new(client, file),
        }
    }
}

#[pyclass(name = FileInfo)]
pub struct PyFileInfo {
    inner: QueryFileGetInfoResponse,
}

#[pymethods]
impl PyQueryFileGetInfo {
    pub fn get(&mut self) -> PyResult<PyFileInfo> {
        let response = self.inner.get().map_err(PyException)?;

        Ok(PyFileInfo {
            inner: response
        })
    }

    pub fn cost(&mut self) -> PyResult<u64> {
        self.inner.cost().map_err(PyException)
    }
}

#[pymethods]
impl PyFileInfo {
    #[getter]
    pub fn file_id(&mut self) -> PyResult<String> {
        Ok(self.inner.file_id.to_string())
    }

    #[getter]
    pub fn size(&mut self) -> PyResult<i64> {
        Ok(self.inner.size)
    }

    #[getter]
    pub fn deleted(&mut self) -> PyResult<bool> {
        Ok(self.inner.deleted)
    }

    #[getter]
    pub fn keys(&mut self) -> PyResult<String> {
        Ok(self.inner.keys.iter().map(|key| key.to_string()).collect())
    }

    pub fn expiration_time(&mut self, py: Python) -> PyResult<Py<PyDateTime>> {
        py_date_time(self.inner.expiration_time, py)
    }
}

