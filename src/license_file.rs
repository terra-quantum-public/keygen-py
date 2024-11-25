use crate::date::Date;
use crate::utils::create_interface;
use keygen_rs::license_file::LicenseFile as KeygenRsLicenseFile;
use pyo3::prelude::*;
use crate::errors::KeygenError;

#[pymodule(name = "license_file")]
pub fn license_file_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Hack: workaround for https://github.com/PyO3/pyo3/issues/759
    Python::with_gil(|py| {
        py.import("sys")?
            .getattr("modules")?
            .set_item("keygen_sh.license_file", m)
    })?;

    m.add_class::<LicenseFile>()?;
    Ok(())
}

create_interface!(LicenseFile, KeygenRsLicenseFile);

#[pymethods]
impl LicenseFile {
    #[getter]
    pub fn id(&self) -> PyResult<String> {
        Ok(self.inner.id.clone())
    }

    #[getter]
    pub fn certificate(&self) -> PyResult<String> {
        Ok(self.inner.certificate.clone())
    }

    #[getter]
    pub fn issued(&self) -> PyResult<Date> {
        Ok(Date::from(self.inner.issued))
    }

    #[getter]
    pub fn expiry(&self) -> PyResult<Date> {
        Ok(Date::from(self.inner.expiry))
    }

    #[getter]
    pub fn ttl(&self) -> PyResult<i32> {
        Ok(self.inner.ttl)
    }

    #[staticmethod]
    fn build_from_cert(test: String, content: String) -> PyResult<Self> {
        match KeygenRsLicenseFile::from_cert(&test, &content) {
            Ok(lf) => Ok(Self { inner: lf }),
            Err(e) => Err(KeygenError::from_error(e))
        }
    }

    fn verify(&self) -> PyResult<()> {
        match self.inner.verify() {
            Ok(_) => Ok(()),
            Err(e) => Err(KeygenError::from_error(e)),
        }
    }

    // TODO proper return LicenseFileDataset
    fn decrypt(&self, key: String) -> PyResult<()> {
        match self.inner.decrypt(&key) {
            Ok(_) => Ok(()),
            Err(e) => Err(KeygenError::from_error(e)),
        }
    }

    // TODO proper return Certificate
    fn parse_certificate(&self) -> PyResult<()> {
        match self.inner.certificate() {
            Ok(_) => Ok(()),
            Err(e) => Err(KeygenError::from_error(e)),
        }
    }
}