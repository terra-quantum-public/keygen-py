use crate::date::Date;
use crate::entitlement::Entitlement;
use crate::machine::Machine;
use crate::utils::{create_interface, pylist_to_string_slice};
use keygen_rs;
use keygen_rs::license::License as KeygenRsLicense;
use pyo3::prelude::*;
use pyo3::types::PyList;
use crate::component::Component;
use crate::errors::KeygenError;
use crate::license_file::LicenseFile;

#[pymodule(name = "license")]
pub fn license_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Hack: workaround for https://github.com/PyO3/pyo3/issues/759
    Python::with_gil(|py| {
        py.import("sys")?
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
    fn id(&self) -> PyResult<String> {
        Ok(self.inner.id.clone())
    }

    #[getter]
    fn key(&self) -> PyResult<String> {
        Ok(self.inner.key.clone())
    }

    #[getter]
    fn name(&self) -> PyResult<Option<String>> {
        Ok(self.inner.name.clone())
    }

    #[getter]
    fn expiry(&self) -> PyResult<Option<Date>> {
        Ok(self.inner.expiry.map(Date::from))
    }

    #[getter]
    fn status(&self) -> PyResult<Option<String>> {
        Ok(self.inner.status.clone())
    }

    #[getter]
    fn policy(&self) -> PyResult<Option<String>> {
        Ok(self.inner.policy.clone())
    }

    #[getter]
    fn scheme(&self) -> PyResult<Option<SchemeCode>> {
        Ok(match self.inner.scheme {
            Some(keygen_rs::license::SchemeCode::Ed25519Sign) => Some(SchemeCode::Ed25519Sign),
            _ => None
        })
    }

    #[pyo3(signature = (fingerprints=None, entitlements=None))]
    fn validate<'a>(&'a self, py: Python<'a>, fingerprints: Option<Bound<'a, PyList>>, entitlements: Option<Bound<'a, PyList>>) -> PyResult<Bound<PyAny>> {
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

    #[pyo3(signature = (fingerprints=None, entitlements=None))]
    fn validate_key<'a>(&'a self, py: Python<'a>, fingerprints: Option<Bound<'a, PyList>>, entitlements: Option<Bound<'a, PyList>>) -> PyResult<Bound<PyAny>> {
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

    fn verify(&self) -> PyResult<Vec<u8>> {
        match self.inner.verify() {
            Ok(resp) => Ok(resp),
            Err(e) => Err(KeygenError::from_error(e)),
        }
    }

    fn activate<'a>(&'a self, py: Python<'a>, fingerprint: String, components: Vec<Component>) -> PyResult<Bound<PyAny>> {
        let my_struct = self.clone();
        let components = components.iter().map(|c| {
            c.clone().into()
        }).collect::<Vec<keygen_rs::component::Component>>();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = my_struct.inner.activate(&fingerprint, &components).await;
            match result {
                Ok(machine) => Ok(Machine::from(machine)),
                Err(e) => Err(KeygenError::from_error(e)),
            }
        })
    }

    fn deactivate<'a>(&'a self, py: Python<'a>, id: String) -> PyResult<Bound<PyAny>> {
        let my_struct = self.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = my_struct.inner.deactivate(&id).await;
            match result {
                Ok(()) => Ok(()),
                Err(e) => Err(KeygenError::from_error(e)),
            }
        })
    }

    fn machine<'a>(&'a self, py: Python<'a>, id: String) -> PyResult<Bound<PyAny>> {
        let my_struct = self.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = my_struct.inner.machine(&id).await;
            match result {
                Ok(machine) => Ok(Machine::from(machine)),
                Err(e) => Err(KeygenError::from_error(e)),
            }
        })
    }

    fn machines<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<PyAny>> {
        let my_struct = self.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = my_struct.inner.machines().await;
            match result {
                Ok(machines) => {
                    Ok(machines.iter().map(|m| {
                        Machine::from(m.clone())
                    }).collect::<Vec<Machine>>())
                },
                Err(e) => Err(KeygenError::from_error(e)),
            }
        })
    }

    fn entitlements<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<PyAny>> {
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
                    }).collect::<Vec<Entitlement>>())
                },
                Err(e) => Err(KeygenError::from_error(e)),
            }
        })
    }

    #[pyo3(signature = (ttl=None, include=None))]
    fn checkout<'a>(&'a self, py: Python<'a>, ttl: Option<i64>, include: Option<Vec<String>>) -> PyResult<Bound<PyAny>> {
        let my_struct = self.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = my_struct.inner.checkout(&keygen_rs::license::LicenseCheckoutOpts { ttl, include }).await;
            match result {
                Ok(lf) => {
                    Ok(LicenseFile::from(lf))
                },
                Err(e) => Err(KeygenError::from_error(e)),
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

