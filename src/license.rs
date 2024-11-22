use crate::date::Date;
use crate::machine::Machine;
use crate::utils::{create_interface, pylist_to_string_slice};
use keygen_rs;
use keygen_rs::license::License as KeygenRsLicense;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyInt, PyList};
use crate::entitlement::Entitlement;

#[pymodule(name = "license")]
pub fn license_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Hack: workaround for https://github.com/PyO3/pyo3/issues/759
    Python::with_gil(|py| {
        py.import_bound("sys")?
            .getattr("modules")?
            .set_item("keygen_sh.license", m)
    })?;

    m.add_class::<SchemeCode>()?;
    m.add_class::<License>()?;
    Ok(())
}

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, PartialEq)]
pub enum SchemeCode {
    Ed25519Sign,
}

create_interface!(License, KeygenRsLicense);

pub struct LicenseCheckoutOpts {}

#[pymethods]
impl License {
    #[getter]
    pub fn id(&self) -> PyResult<String> {
        Ok(self.inner.id.clone())
    }

    #[getter]
    pub fn key(&self) -> PyResult<String> {
        Ok(self.inner.key.clone())
    }

    #[getter]
    pub fn name(&self) -> PyResult<Option<String>> {
        Ok(self.inner.name.clone())
    }

    #[getter]
    pub fn expiry(&self) -> PyResult<Option<Date>> {
        Ok(self.inner.expiry.map(Date::from))
    }

    #[getter]
    pub fn status(&self) -> PyResult<Option<String>> {
        Ok(self.inner.status.clone())
    }

    #[getter]
    pub fn policy(&self) -> PyResult<Option<String>> {
        Ok(self.inner.policy.clone())
    }

    #[getter]
    pub fn scheme(&self) -> PyResult<Option<SchemeCode>> {
        Ok(match self.inner.scheme {
            Some(keygen_rs::license::SchemeCode::Ed25519Sign) => Some(SchemeCode::Ed25519Sign),
            _ => None
        })
    }

    #[pyo3(signature = (fingerprints=None, entitlements=None))]
    fn validate<'a>(&self, py: Python<'a>, fingerprints: Option<Bound<'a, PyList>>, entitlements: Option<Bound<'a, PyList>>) -> PyResult<Bound<PyAny>> {
        let fingerprints = fingerprints.unwrap_or_else(|| PyList::empty_bound(py));
        let entitlements = entitlements.unwrap_or_else(|| PyList::empty_bound(py));

        let fingerprints_vec = pylist_to_string_slice(fingerprints)?;
        let entitlements_vec = pylist_to_string_slice(entitlements)?;

        pyo3_async_runtimes::tokio::future_into_py::<License>(py, async move {
            let result = keygen_rs::validate(&fingerprints_vec, &entitlements_vec).await;

            match result {
                Ok(license) => Ok(License::from(license)),
                Err(e) => {
                    Err(PyRuntimeError::new_err(e.to_string()))
                },
            }
        })
    }

    #[pyo3(signature = (fingerprints=None, entitlements=None))]
    fn validate_key<'a>(&self, py: Python<'a>, fingerprints: Option<Bound<'a, PyList>>, entitlements: Option<Bound<'a, PyList>>) -> PyResult<Bound<PyAny>> {
        let fingerprints = fingerprints.unwrap_or_else(|| PyList::empty_bound(py));
        let entitlements = entitlements.unwrap_or_else(|| PyList::empty_bound(py));

        let fingerprints_vec = pylist_to_string_slice(fingerprints)?;
        let entitlements_vec = pylist_to_string_slice(entitlements)?;

        pyo3_async_runtimes::tokio::future_into_py::<License>(py, async move {
            let result = keygen_rs::validate(&fingerprints_vec, &entitlements_vec).await;

            match result {
                Ok(license) => Ok(License::from(license)),
                Err(e) => {
                    Err(PyRuntimeError::new_err(e.to_string()))
                },
            }
        })
    }

    fn verify(&self) -> PyResult<Vec<u8>> {
        match self.inner.verify() {
            Ok(resp) => Ok(resp),
            Err(e) => {
                Err(PyRuntimeError::new_err(e.to_string()))
            },
        }
    }

    fn activate(&self, py: Python, fingerprint: String, components: Bound<PyList<PyAny>>) -> PyResult<Bound<PyAny>> {
        let my_struct = self.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = my_struct.inner.activate(&fingerprint, &components).await;
            match result {
                Ok((machine)) => Ok(Machine::from(machine)),
                Err(e) => {
                    Err(PyRuntimeError::new_err(e.to_string()))
                },
            }
        })
    }

    fn deactivate(&self, py: Python, id: String) -> PyResult<Bound<PyAny>> {
        let my_struct = self.clone();

        pyo3_async_runtimes::tokio::future_into_py::<()>(py, async move {
            let result = my_struct.inner.deactivate(&id).await;
            match result {
                Ok(()) => Ok(()),
                Err(e) => {
                    Err(PyRuntimeError::new_err(e.to_string()))
                },
            }
        })
    }

    fn machine(&self, py: Python, id: String) -> PyResult<Bound<PyAny>> {
        let my_struct = self.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = my_struct.inner.machine(&id).await;
            match result {
                Ok(machine) => {
                    Ok(Machine::from(machine))
                },
                Err(e) => {
                    Err(PyRuntimeError::new_err(e.to_string()))
                },
            }
        })
    }

    fn machines(&self, py: Python) -> PyResult<Bound<PyAny>> {
        let my_struct = self.clone();

        pyo3_async_runtimes::tokio::future_into_py::<Machine>>(py, async move {
            let result = my_struct.inner.machines().await;
            match result {
                Ok(machines) => {
                    Ok(machines.iter().map(|m| {
                        Machine::from(m.clone())
                    }).collect())
                },
                Err(e) => {
                    Err(PyRuntimeError::new_err(e.to_string()))
                },
            }
        })
    }

    fn entitlements(&self, py: Python) -> PyResult<Bound<PyAny>> {
        let my_struct = self.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = my_struct.inner.entitlements().await;
            match result {
                Ok(entitlements) => {
                    Ok(entitlements.iter().map(|entitlement| {
                        let e = entitlement.clone();
                        Entitlement::build(
                            e.id,
                            e.name,
                            e.code,
                            e.created,
                            e.updated
                        )
                    }).collect())
                },
                Err(e) => {
                    Err(PyRuntimeError::new_err(e.to_string()))
                },
            }
        })
    }

    fn checkout(&self, py: Python, ttl: Option<i64>, include: Option<Vec<String>>) -> PyResult<Bound<PyAny>> {
        let my_struct = self.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = my_struct.inner.checkout(&keygen_rs::license::LicenseCheckoutOpts { ttl, include }).await;
            match result {
                Ok(lf) => {
                    // TODO Need to implement this once we have licenseFile type
                    Ok(())
                },
                Err(e) => {
                    Err(PyRuntimeError::new_err(e.to_string()))
                },
            }
        })
    }
}

impl From<SchemeCode> for keygen_rs::license::SchemeCode {
    fn from(val: SchemeCode) -> Self {
        match val {
            SchemeCode::Ed25519Sign => keygen_rs::license::SchemeCode::Ed25519Sign,
        }
    }
}

