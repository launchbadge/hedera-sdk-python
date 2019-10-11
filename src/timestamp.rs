use chrono::{prelude::*, DateTime, Utc, NaiveDateTime, NaiveDate, NaiveTime};
use derive_more::{From, Into};
use pyo3::{prelude::*, types::{PyDateTime, PyTimeAccess, PyDateAccess}};
use try_from::{TryFrom};
use pyo3::types::PyTzInfo;

#[pyclass(name = Timestamp)]
#[derive(From, Into)]
pub struct PyTimestamp {
    pub(crate) inner: Py<PyDateTime>
}

#[pymethods]
impl PyTimestamp {
    #[new]
    pub fn __new__(obj: &PyRawObject, ts: f64) -> PyResult<()> {
        let py = unsafe { Python::assume_gil_acquired() };
        let datetime = py.import("datetime").map_err(|e| e.print(py)).unwrap();
        let timezone = datetime.get("timezone").unwrap();
        let utc = FromPyObject::extract(timezone.getattr("utc").unwrap())?: &PyTzInfo;
        let dt = PyDateTime::from_timestamp(py, ts, Some(utc))?;
        obj.init(move || Self {
            inner: dt
        })
    }
}

impl TryFrom<f64> for PyTimestamp {
    type Err = PyErr;

    fn try_from(ts: f64) -> PyResult<PyTimestamp> {
        // note: GIL should be acquired by this point and we don't know a safer way to get a py
        //       instance from inside a `#[getter]`
        let py = unsafe { Python::assume_gil_acquired() };

        Ok(
            PyTimestamp{
                inner: PyDateTime::from_timestamp(py, ts, None)?
            }
        )
    }
}

impl TryFrom<DateTime<Utc>> for PyTimestamp {
    type Err = PyErr;

    fn try_from(dt: DateTime<Utc>) -> PyResult<PyTimestamp> {
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

        Ok(
            PyTimestamp{
                inner: PyDateTime::new(
                    py,
                    year,
                    month,
                    day,
                    hour,
                    minute,
                    second,
                    microsecond,
                    None, )?
            }
        )
    }
}

impl Into<DateTime<Utc>> for &PyTimestamp {
    fn into(self) -> DateTime<Utc> {
        let py = unsafe { Python::assume_gil_acquired() };

        let date = NaiveDate::from_ymd(
            self.inner.as_ref(py).get_year(),
            self.inner.as_ref(py).get_month() as u32,
            self.inner.as_ref(py).get_day() as u32
        );

        let time = NaiveTime::from_hms_micro(
            self.inner.as_ref(py).get_hour() as u32,
            self.inner.as_ref(py).get_minute() as u32,
            self.inner.as_ref(py).get_second() as u32,
            self.inner.as_ref(py).get_microsecond()
        );

        DateTime::<Utc>::from_utc(
            NaiveDateTime::new(date, time),
            Utc
        )
    }
}
