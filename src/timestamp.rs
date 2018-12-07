use chrono::{prelude::*, DateTime, Utc};
use pyo3::{prelude::*, types::PyDateTime};

#[allow(clippy::cast_possible_truncation)]
pub(crate) fn py_date_time(dt: DateTime<Utc>, py: Python) -> PyResult<Py<PyDateTime>> {
    let year = dt.year();
    let month = dt.month() as u8; // 1..=12
    let day = dt.day() as u8; // 1..=31
    let hour = dt.hour() as u8; // 0..=23
    let minute = dt.minute() as u8; // 0..=59
    let second = dt.second() as u8; // 0..=59
    let microsecond = (dt.nanosecond() % 1_000_000_000) / 1_000;

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
