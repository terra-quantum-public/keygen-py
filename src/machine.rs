use crate::date::Date;
use crate::utils::create_interface;
use keygen_rs::machine::Machine as KeygenRsMachine;
use pyo3::prelude::{PyAnyMethods, PyModule, PyModuleMethods};
use pyo3::{pyclass, pymethods, pymodule, Bound, PyResult, Python};

#[pymodule(name = "machine")]
pub fn machine_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Hack: workaround for https://github.com/PyO3/pyo3/issues/759
    Python::with_gil(|py| {
        py.import("sys")?
            .getattr("modules")?
            .set_item("keygen_sh.machine", m)
    })?;

    m.add_class::<Machine>()?;
    Ok(())
}

create_interface!(Machine, KeygenRsMachine);

#[pymethods]
impl Machine {
    #[getter]
    pub fn id(&self) -> PyResult<String> {
        Ok(self.inner.id.clone())
    }

    #[getter]
    pub fn fingerprint(&self) -> PyResult<String> {
        Ok(self.inner.fingerprint.clone())
    }

    #[getter]
    pub fn name(&self) -> PyResult<Option<String>> {
        Ok(self.inner.name.clone())
    }


    #[getter]
    pub fn platform(&self) -> PyResult<Option<String>> {
        Ok(self.inner.platform.clone())
    }

    #[getter]
    pub fn hostname(&self) -> PyResult<Option<String>> {
        Ok(self.inner.hostname.clone())
    }

    #[getter]
    pub fn cores(&self) -> PyResult<Option<i32>> {
        Ok(self.inner.cores)
    }

    #[getter]
    pub fn require_heartbeat(&self) -> PyResult<bool> {
        Ok(self.inner.require_heartbeat)
    }

    #[getter]
    pub fn heartbeat_status(&self) -> PyResult<String> {
        Ok(self.inner.heartbeat_status.clone())
    }

    #[getter]
    pub fn heartbeat_duration(&self) -> PyResult<Option<i32>> {
        Ok(self.inner.heartbeat_duration)
    }

    #[getter]
    pub fn created(&self) -> PyResult<Date> {
        Ok(Date::from(self.inner.created))
    }

    #[getter]
    pub fn updated(&self) -> PyResult<Date> {
        Ok(Date::from(self.inner.updated))
    }
}