use hedera::{
    query::{Query, QueryFileGetInfo, QueryFileGetInfoResponse},
    Client, FileId,
};
use super::timestamp::py_date_time;
use pyo3::prelude::*;
use pyo3::types::PyDateTime;
use crate::errors::PyException;
use chrono::DateTime;
use chrono::Utc;

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
pub struct FileInfo {
    #[prop(get)]
    pub file_id: String,
    #[prop(get)]
    pub size: i64,
    expiration_time: DateTime<Utc>,
    #[prop(get)]
    pub deleted: bool,
    #[prop(get)]
    pub keys: Vec<String>,
}

#[pymethods]
impl PyQueryFileGetInfo {
    pub fn get(&mut self) -> PyResult<FileInfo> {
        let response = self.inner.get().map_err(PyException)?;

        Ok(FileInfo {
            file_id: response.file_id.to_string(),
            size: response.size,
            expiration_time: response.expiration_time,
            deleted: response.deleted,
            keys: response.keys.into_iter().map(|key| key.to_string()).collect()
        })
    }

    pub fn cost(&mut self) -> PyResult<u64> {
        self.inner.cost().map_err(PyException)
    }
}

#[pymethods]
impl FileInfo {
    pub fn expiration_time(&mut self, py: Python) -> PyResult<Py<PyDateTime>> {
        py_date_time(self.expiration_time, py)
    }

}

