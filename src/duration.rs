use pyo3::{prelude::*, types::PyDelta};
use std::time::Duration;
use try_from::TryFrom;
use derive_more::{From, Into};
use pyo3::types::PyDeltaAccess;

#[pyclass(name = Duration)]
#[derive(From, Into)]
pub struct PyDuration{
    pub(crate) inner: Py<PyDelta>
}

//impl IntoPyObject for PyDuration {
//    fn into_object(self, py: Python) -> PyObject {
//        self.0.into_object(py)
//    }
//}

#[pymethods]
impl PyDuration {
    #[new]
    pub fn __new__(obj: &PyRawObject, days: i32, seconds: i32, micos: i32) -> PyResult<()> {
        let py = unsafe { Python::assume_gil_acquired() };
        let d = PyDelta::new(py, days, seconds, micos, false)?;
        obj.init(move || Self {
            inner: d
        })
    }
}

impl TryFrom<Duration> for PyDuration {
    type Err = PyErr;

    fn try_from(duration: Duration) -> PyResult<PyDuration> {
        // note: GIL should be acquired by this point and we don't know a safer way to get a py
        //       instance from inside a `#[getter]`
        let py = unsafe { Python::assume_gil_acquired() };

        let seconds = duration.as_secs() as i32;
        let microseconds = duration.subsec_micros() as i32;

        Ok(
            PyDuration{
                inner: PyDelta::new(
                    py,
                    0,
                    seconds,
                    microseconds,
                    false,)?
            }
        )
    }
}

impl Into<Duration> for &PyDuration {
    fn into(self) -> Duration {
        let py = unsafe { Python::assume_gil_acquired() };

        let days_seconds = self.inner.as_ref(py).get_days() * 86400;
        let seconds = self.inner.as_ref(py).get_seconds();
        let micros = self.inner.as_ref(py).get_microseconds();

        Duration::new((seconds + days_seconds) as u64, (micros * 1000) as u32)
    }
}
