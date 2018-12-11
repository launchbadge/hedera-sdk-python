use pyo3::{prelude::*, types::PyDelta};
use std::time::Duration;
use try_from::TryFrom;

pub struct PyDuration(Py<PyDelta>);

impl IntoPyObject for PyDuration {
    fn into_object(self, py: Python) -> PyObject {
        self.0.into_object(py)
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

        Ok(PyDuration(PyDelta::new(
            py,
            0,
            seconds,
            microseconds,
            false,
        )?))
    }
}
