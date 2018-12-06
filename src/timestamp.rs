use chrono::{prelude::*, DateTime, Utc};
use pyo3::{prelude::*, types::PyDateTime};

pub(crate) fn py_date_time(dt: DateTime<Utc>, py: Python) -> PyResult<Py<PyDateTime>> {
    let year = dt.year();
    let month = dt.month() as u8;
    let day = dt.day() as u8;
    let hour = dt.hour() as u8;
    let minute = dt.minute() as u8;
    let second = dt.second() as u8;
    let microsecond = dt.timestamp_subsec_micros();

    PyDateTime::new(
        py,
        year,
        month,
        day,
        hour,
        minute,
        second,
        microsecond,
        None,
    )
}
