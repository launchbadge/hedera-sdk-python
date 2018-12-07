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

#[pymethods]
impl PyQueryFileGetInfo {
    #[getter]
    pub fn file_id(&mut self) -> PyResult<String> {
        let response = self.inner.get().map_err(PyException)?;
        Ok(response.file_id.to_string())
    }

    #[getter]
    pub fn size(&mut self) -> PyResult<i64> {
        let response = self.inner.get().map_err(PyException)?;
        Ok(response.size)
    }

    #[getter]
    pub fn deleted(&mut self) -> PyResult<bool> {
        let response = self.inner.get().map_err(PyException)?;
        Ok(response.deleted)
    }

    #[getter]
    pub fn keys(&mut self) -> PyResult<String> {
        let response = self.inner.get().map_err(PyException)?;
        Ok(response.keys.into_iter().map(|key| key.to_string()).collect())
    }

    pub fn expiration_time(&mut self, py: Python) -> PyResult<Py<PyDateTime>> {
        let response = self.inner.get().map_err(PyException)?;
        py_date_time(response.expiration_time, py)
    }

    pub fn cost(&mut self) -> PyResult<u64> {
        self.inner.cost().map_err(PyException)
    }
}

