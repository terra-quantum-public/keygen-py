use chrono::{DateTime, Utc};
use pyo3::{IntoPy, Py, PyAny, PyObject, Python};

#[derive(Debug, Clone, PartialEq)]
pub struct Date(DateTime<Utc>);

impl IntoPy<Py<PyAny>> for Date {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.0.to_rfc3339().into_py(py)
    }
}

impl From<DateTime<Utc>> for Date {
    fn from(value: DateTime<Utc>) -> Self {
        Date(value)
    }
}