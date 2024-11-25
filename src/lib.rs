use crate::license::{License, SchemeCode};
use crate::utils::pylist_to_string_slice;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::PyModuleMethods;
use pyo3::types::{PyList, PyModule};
use pyo3::{pyfunction, pymodule, wrap_pyfunction, wrap_pymodule, Bound, PyAny, PyResult, Python};

pub(crate) mod date;
pub(crate) mod utils;
pub mod config;
pub mod entitlement;
pub mod license;
pub mod license_file;
pub mod machine;

#[pyfunction]
fn verify(scheme: SchemeCode, signed_key: &str) -> PyResult<String> {
    match keygen_rs::verify(scheme.into(), signed_key) {
        Ok(data) => Ok(String::from_utf8_lossy(&data).to_string()),
        Err(e) => Err(PyRuntimeError::new_err(e.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (fingerprints=None, entitlements=None))]
fn validate<'a>(py: Python<'a>, fingerprints: Option<Bound<'a, PyList>>, entitlements: Option<Bound<'a, PyList>>) -> PyResult<Bound<'a, PyAny>> {
    let fingerprints = fingerprints.unwrap_or_else(|| PyList::empty_bound(py));
    let entitlements = entitlements.unwrap_or_else(|| PyList::empty_bound(py));

    let fingerprints_vec = pylist_to_string_slice(fingerprints)?;
    let entitlements_vec = pylist_to_string_slice(entitlements)?;

    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let result = keygen_rs::validate(&fingerprints_vec, &entitlements_vec).await;

        match result {
            Ok(license) => Ok(License::from(license)),
            Err(e) => {
                Err(PyRuntimeError::new_err(e.to_string()))
            },
        }
    })
}

#[pymodule]
fn keygen_sh(_: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(config::config_module))?;
    m.add_wrapped(wrap_pymodule!(license::license_module))?;
    m.add_wrapped(wrap_pymodule!(machine::machine_module))?;
    m.add_wrapped(wrap_pymodule!(entitlement::entitlement_module))?;
    m.add_wrapped(wrap_pymodule!(license_file::license_file_module))?;

    m.add_function(wrap_pyfunction!(validate, m)?)?;
    m.add_function(wrap_pyfunction!(verify, m)?)?;
    Ok(())
}