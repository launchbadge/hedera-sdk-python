use chrono::{prelude::*, DateTime, Utc};
use derive_more::From;
use pyo3::{prelude::*, types};
use try_from::TryFrom;

#[derive(From)]
pub struct PyDateTime(Py<types::PyDateTime>);

impl IntoPyObject for PyDateTime {
    fn into_object(self, py: Python) -> PyObject {
        self.0.into_object(py)
    }
}

impl TryFrom<DateTime<Utc>> for PyDateTime {
    type Err = PyErr;

    fn try_from(dt: DateTime<Utc>) -> PyResult<PyDateTime> {
        // note: GIL should be acquired by this point and we don't know a safer way to get a py
        //       instance from inside a `#[getter]`
        let py = unsafe { Python::assume_gil_acquired() };

        let year = dt.year();
        let month = dt.month() as u8; // 1..=12
        let day = dt.day() as u8; // 1..=31
        let hour = dt.hour() as u8; // 0..=23
        let minute = dt.minute() as u8; // 0..=59
        let second = dt.second() as u8; // 0..=59
        let microsecond = (dt.nanosecond() % 1_000_000_000) / 1_000;

        Ok(PyDateTime(types::PyDateTime::new(
            py,
            year,
            month,
            day,
            hour,
            minute,
            second,
            microsecond,
            None,
        )?))
    }
}
