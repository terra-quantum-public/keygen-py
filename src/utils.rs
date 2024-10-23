use pyo3::{Bound, PyResult};
use pyo3::types::{PyAnyMethods, PyList, PyListMethods};

pub fn pylist_to_string_slice(pylist: Bound<PyList>) -> PyResult<Vec<String>> {
    let mut strings = Vec::new();
    for item in pylist.iter() {
        let s: String = item.extract()?;
        strings.push(s);
    }
    Ok(strings)
}
