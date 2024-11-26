use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyList, PyNone, PyString};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct JsonValue(pub Value);

impl<'py> IntoPyObject<'py> for JsonValue {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            Value::Null => Ok(PyNone::get(py).to_owned().into_any()),
            Value::Bool(b) => Ok(PyBool::new(py, b).to_owned().into_any()),
            Value::Number(num) => {
                if let Some(i) = num.as_i64() {
                    Ok(i.into_pyobject(py)?.into_any())
                } else if let Some(u) = num.as_u64() {
                    Ok(u.into_pyobject(py)?.into_any())
                } else if let Some(f) = num.as_f64() {
                    Ok(f.into_pyobject(py)?.into_any())
                } else {
                    Ok(PyNone::get(py).to_owned().into_any())
                }
            }
            Value::String(s) => Ok(PyString::new(py, &s).into_any()),
            Value::Array(arr) => {
                let py_list = PyList::empty(py);
                for elem in arr {
                    py_list.append(JsonValue(elem).into_pyobject(py)?.into_any())?;
                }
                Ok(py_list.into_any())
            }
            Value::Object(obj) => {
                let py_dict = PyDict::new(py);
                for (key, value) in obj {
                    py_dict.set_item(key, JsonValue(value).into_pyobject(py)?.into_any())?;
                }
                Ok(py_dict.into_any())
            }
        }
    }
}