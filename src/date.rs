use chrono::{DateTime, Utc};
use pyo3::types::PyString;
use pyo3::{Bound, IntoPyObject, PyErr, Python};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Date(DateTime<Utc>);

impl<'py> IntoPyObject<'py> for Date {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let date_str = self.0.to_rfc3339();
        Ok(PyString::new(py, &date_str))
    }
}

impl From<DateTime<Utc>> for Date {
    fn from(value: DateTime<Utc>) -> Self {
        Date(value)
    }
}