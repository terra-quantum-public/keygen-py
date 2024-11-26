use pyo3::types::{PyAnyMethods, PyList, PyListMethods};
use pyo3::{Bound, PyResult};

pub(crate) fn pylist_to_string_slice(pylist: Bound<PyList>) -> PyResult<Vec<String>> {
    let mut strings = Vec::new();
    for item in pylist.iter() {
        let s: String = item.extract()?;
        strings.push(s);
    }
    Ok(strings)
}

macro_rules! create_interface {
    ($name: ident, $type: ident) => {
        #[pyclass(frozen)]
        #[derive(Debug, Clone)]
        pub struct $name {
            inner: $type,
        }

        impl $name {
            pub(crate) fn from(origin: $type) -> Self {
                Self {
                    inner: origin,
                }
            }
        }
    };
}

macro_rules! create_interface_no_clone {
    ($name: ident, $type: ident) => {
        #[pyclass(frozen)]
        #[derive(Debug)]
        pub struct $name {
            inner: $type,
        }

        impl $name {
            pub(crate) fn from(origin: $type) -> Self {
                Self {
                    inner: origin,
                }
            }
        }
    };
}

pub(crate) use create_interface;
pub(crate) use create_interface_no_clone;