use crate::license::{License, SchemeCode};
use crate::utils::pylist_to_string_slice;
use pyo3::prelude::PyModuleMethods;
use pyo3::types::{PyList, PyModule};
use pyo3::{pyfunction, pymodule, wrap_pyfunction, wrap_pymodule, Bound, PyAny, PyResult, Python};
use crate::errors::KeygenError;

pub(crate) mod date;
pub(crate) mod json;
pub(crate) mod utils;
pub mod certificate;
pub mod component;
pub mod config;
pub mod entitlement;
pub mod errors;
pub mod license;
pub mod license_file;
pub mod machine;
pub mod machine_file;

#[pyfunction]
fn verify(scheme: SchemeCode, signed_key: &str) -> PyResult<String> {
    match keygen_rs::verify(scheme.into(), signed_key) {
        Ok(data) => Ok(String::from_utf8_lossy(&data).to_string()),
        Err(e) => Err(KeygenError::from_error(e)),
    }
}

#[pyfunction]
#[pyo3(signature = (fingerprints=None, entitlements=None))]
fn validate<'a>(py: Python<'a>, fingerprints: Option<Bound<'a, PyList>>, entitlements: Option<Bound<'a, PyList>>) -> PyResult<Bound<'a, PyAny>> {
    let fingerprints = fingerprints.unwrap_or_else(|| PyList::empty(py));
    let entitlements = entitlements.unwrap_or_else(|| PyList::empty(py));

    let fingerprints_vec = pylist_to_string_slice(fingerprints)?;
    let entitlements_vec = pylist_to_string_slice(entitlements)?;

    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let result = keygen_rs::validate(&fingerprints_vec, &entitlements_vec).await;

        match result {
            Ok(license) => Ok(License::from(license)),
            Err(e) => Err(KeygenError::from_error(e)),
        }
    })
}

#[pymodule]
fn keygen_sh(_: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(certificate::certificate_module))?;
    m.add_wrapped(wrap_pymodule!(component::component_module))?;
    m.add_wrapped(wrap_pymodule!(config::config_module))?;
    m.add_wrapped(wrap_pymodule!(entitlement::entitlement_module))?;
    m.add_wrapped(wrap_pymodule!(errors::errors_module))?;
    m.add_wrapped(wrap_pymodule!(license::license_module))?;
    m.add_wrapped(wrap_pymodule!(license_file::license_file_module))?;
    m.add_wrapped(wrap_pymodule!(machine::machine_module))?;
    m.add_wrapped(wrap_pymodule!(machine_file::machine_file_module))?;

    m.add_function(wrap_pyfunction!(validate, m)?)?;
    m.add_function(wrap_pyfunction!(verify, m)?)?;
    Ok(())
}