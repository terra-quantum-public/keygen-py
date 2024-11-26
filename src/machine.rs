use crate::date::Date;
use crate::errors::KeygenError;
use crate::machine_file::MachineFile;
use crate::utils::create_interface;
use keygen_rs::machine::Machine as KeygenRsMachine;
use pyo3::prelude::{PyAnyMethods, PyModule, PyModuleMethods};
use pyo3::{pyclass, pymethods, pymodule, Bound, PyAny, PyResult, Python};

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
    fn id(&self) -> PyResult<String> {
        Ok(self.inner.id.clone())
    }

    #[getter]
    fn fingerprint(&self) -> PyResult<String> {
        Ok(self.inner.fingerprint.clone())
    }

    #[getter]
    fn name(&self) -> PyResult<Option<String>> {
        Ok(self.inner.name.clone())
    }

    #[getter]
    fn platform(&self) -> PyResult<Option<String>> {
        Ok(self.inner.platform.clone())
    }

    #[getter]
    fn hostname(&self) -> PyResult<Option<String>> {
        Ok(self.inner.hostname.clone())
    }

    #[getter]
    fn cores(&self) -> PyResult<Option<i32>> {
        Ok(self.inner.cores)
    }

    #[getter]
    fn require_heartbeat(&self) -> PyResult<bool> {
        Ok(self.inner.require_heartbeat)
    }

    #[getter]
    fn heartbeat_status(&self) -> PyResult<String> {
        Ok(self.inner.heartbeat_status.clone())
    }

    #[getter]
    fn heartbeat_duration(&self) -> PyResult<Option<i32>> {
        Ok(self.inner.heartbeat_duration)
    }

    #[getter]
    fn created(&self) -> PyResult<Date> {
        Ok(Date::from(self.inner.created))
    }

    #[getter]
    fn updated(&self) -> PyResult<Date> {
        Ok(Date::from(self.inner.updated))
    }

    fn deactivate<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<PyAny>> {
        let my_struct = self.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = my_struct.inner.deactivate().await;
            match result {
                Ok(_) => Ok(()),
                Err(e) => Err(KeygenError::from_error(e)),
            }
        })
    }

    #[pyo3(signature = (ttl=None, include=None))]
    fn checkout<'a>(
        &'a self,
        py: Python<'a>,
        ttl: Option<i64>,
        include: Option<Vec<String>>,
    ) -> PyResult<Bound<PyAny>> {
        let my_struct = self.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = my_struct
                .inner
                .checkout(&keygen_rs::machine::MachineCheckoutOpts { ttl, include })
                .await;
            match result {
                Ok(mf) => Ok(MachineFile::from(mf)),
                Err(e) => Err(KeygenError::from_error(e)),
            }
        })
    }

    fn ping<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<PyAny>> {
        let my_struct = self.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = my_struct.inner.ping().await;
            match result {
                Ok(m) => Ok(Machine::from(m)),
                Err(e) => Err(KeygenError::from_error(e)),
            }
        })
    }
}
