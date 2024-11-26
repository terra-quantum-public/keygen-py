use pyo3::prelude::*;
use pyo3::{pymethods, pymodule, Bound, PyResult, Python};
use crate::utils::{create_interface, create_interface_no_clone};
use keygen_rs::machine_file::MachineFile as KeygenRsMachineFile;
use keygen_rs::machine_file::MachineFileDataset as KeygenRsMachineFileDataset;
use crate::certificate::Certificate;
use crate::errors::KeygenError;

#[pymodule(name = "machine_file")]
pub fn machine_file_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Hack: workaround for https://github.com/PyO3/pyo3/issues/759
    Python::with_gil(|py| {
        py.import("sys")?
            .getattr("modules")?
            .set_item("keygen_sh.machine_file", m)
    })?;

    m.add_class::<MachineFile>()?;
    Ok(())
}

create_interface!(MachineFile, KeygenRsMachineFile);
create_interface_no_clone!(MachineFileDataset, KeygenRsMachineFileDataset);

#[pymethods]
impl MachineFile {
    #[staticmethod]
    pub fn from_cert(key: String, content: String) -> PyResult<Self> {
        match KeygenRsMachineFile::from_cert(&key, &content) {
            Ok(mf) => Ok(MachineFile::from(mf)),
            Err(e) => Err(KeygenError::from_error(e)),
        }
    }

    pub fn verify(&self) -> PyResult<()> {
        match self.inner.verify() {
            Ok(_) => Ok(()),
            Err(e) => Err(KeygenError::from_error(e)),
        }
    }

    pub fn decrypt(&self, key: String) -> PyResult<MachineFileDataset> {
        match self.inner.decrypt(&key) {
            Ok(mfd) => Ok(MachineFileDataset::from(mfd)),
            Err(e) => Err(KeygenError::from_error(e)),
        }
    }

    pub fn certificate(&self) -> PyResult<Certificate> {
        match self.inner.certificate() {
            Ok(mfd) => Ok(Certificate::build(mfd.enc, mfd.sig, mfd.alg)),
            Err(e) => Err(KeygenError::from_error(e)),
        }
    }
}