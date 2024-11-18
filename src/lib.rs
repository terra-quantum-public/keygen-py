use crate::config::KeygenConfig;
use pyo3::types::{PyList, PyModule};
use pyo3::{pyfunction, pymodule, wrap_pyfunction, Bound, PyAny, PyResult, Python};
use pyo3::exceptions::PyRuntimeError;
use crate::license::{License, SchemeCode};
use crate::utils::pylist_to_string_slice;

pub mod license;
pub mod date;
mod config;
mod utils;

#[pyfunction]
fn set_config(config: KeygenConfig) -> PyResult<()> {
    keygen_rs::config::set_config(config.into());
    Ok(())
}

#[pyfunction]
fn verify(scheme: SchemeCode, signed_key: &str) -> PyResult<String> {
    match keygen_rs::verify(scheme.into(), signed_key) {
        Ok(data) => Ok(String::from_utf8_lossy(&data).to_string()),
        Err(e) => Err(PyRuntimeError::new_err(e.to_string())),
    }
}

#[pyfunction]
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
    m.add_function(wrap_pyfunction!(set_config, m)?)?;
    m.add_function(wrap_pyfunction!(validate, m)?)?;
    m.add_function(wrap_pyfunction!(verify, m)?)?;
    m.add_class::<SchemeCode>()?;
    m.add_class::<KeygenConfig>()?;
    m.add_class::<License>()?;
    Ok(())
}