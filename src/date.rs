use chrono::{DateTime, Datelike, Timelike, Utc};
use pyo3::types::PyDateTime;
use pyo3::{Bound, IntoPyObject, PyErr, Python};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Date(DateTime<Utc>);

impl<'py> IntoPyObject<'py> for Date {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let datetime = self.0;

        PyDateTime::new(
            py,
            datetime.year(),
            datetime.month() as u8,
            datetime.day() as u8,
            datetime.hour() as u8,
            datetime.minute() as u8,
            datetime.second() as u8,
            datetime.timestamp_subsec_micros(),
            None
        )
    }
}

impl From<DateTime<Utc>> for Date {
    fn from(value: DateTime<Utc>) -> Self {
        Date(value)
    }
}
