use pyo3::prelude::*;
use pyo3::{pymethods, pymodule, Bound, PyResult, Python};
use crate::utils::{create_interface, create_interface_no_clone};
use keygen_rs::machine_file::MachineFile as KeygenRsMachineFile;
use keygen_rs::machine_file::MachineFileDataset as KeygenRsMachineFileDataset;
use crate::certificate::Certificate;
use crate::date::Date;
use crate::errors::KeygenError;
use crate::license::License;
use crate::machine::Machine;

#[pymodule(name = "machine_file")]
pub fn machine_file_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Hack: workaround for https://github.com/PyO3/pyo3/issues/759
    Python::with_gil(|py| {
        py.import("sys")?
            .getattr("modules")?
            .set_item("keygen_sh.machine_file", m)
    })?;

    m.add_class::<MachineFile>()?;
    m.add_class::<MachineFileDataset>()?;
    Ok(())
}

create_interface!(MachineFile, KeygenRsMachineFile);
create_interface_no_clone!(MachineFileDataset, KeygenRsMachineFileDataset);

#[pymethods]
impl MachineFile {
    #[getter]
    fn id(&self) -> PyResult<String> {
        Ok(self.inner.id.clone())
    }

    #[getter]
    fn certificate(&self) -> PyResult<String> {
        Ok(self.inner.certificate.clone())
    }

    #[getter]
    fn issued(&self) -> PyResult<Date> {
        Ok(Date::from(self.inner.issued))
    }

    #[getter]
    fn expiry(&self) -> PyResult<Date> {
        Ok(Date::from(self.inner.expiry))
    }

    #[getter]
    fn ttl(&self) -> PyResult<i32> {
        Ok(self.inner.ttl)
    }
    
    #[staticmethod]
    fn from_cert(key: String, content: String) -> PyResult<Self> {
        match KeygenRsMachineFile::from_cert(&key, &content) {
            Ok(mf) => Ok(MachineFile::from(mf)),
            Err(e) => Err(KeygenError::from_error(e)),
        }
    }

    fn verify(&self) -> PyResult<()> {
        match self.inner.verify() {
            Ok(_) => Ok(()),
            Err(e) => Err(KeygenError::from_error(e)),
        }
    }

    fn decrypt(&self, key: String) -> PyResult<MachineFileDataset> {
        match self.inner.decrypt(&key) {
            Ok(mfd) => Ok(MachineFileDataset::from(mfd)),
            Err(e) => Err(KeygenError::from_error(e)),
        }
    }

    fn build_certificate(&self) -> PyResult<Certificate> {
        match self.inner.certificate() {
            Ok(mfd) => Ok(Certificate::build(mfd.enc, mfd.sig, mfd.alg)),
            Err(e) => Err(KeygenError::from_error(e)),
        }
    }
}

#[pymethods]
impl MachineFileDataset {
    #[getter]
    fn license(&self) -> PyResult<License> {
        Ok(License::from(self.inner.license.clone()))
    }

    #[getter]
    fn machine(&self) -> PyResult<Machine> {
        Ok(Machine::from(self.inner.machine.clone()))
    }

    #[getter]
    fn issued(&self) -> PyResult<Date> {
        Ok(Date::from(self.inner.issued))
    }

    #[getter]
    fn expiry(&self) -> PyResult<Date> {
        Ok(Date::from(self.inner.expiry))
    }

    #[getter]
    fn ttl(&self) -> PyResult<i32> {
        Ok(self.inner.ttl)
    }
}