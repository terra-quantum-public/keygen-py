use crate::date::Date;
use crate::utils::create_interface;
use keygen_rs::license_file::LicenseFile as KeygenRsLicenseFile;
use pyo3::prelude::*;

#[pymodule(name = "license_file")]
pub fn license_file_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Hack: workaround for https://github.com/PyO3/pyo3/issues/759
    Python::with_gil(|py| {
        py.import_bound("sys")?
            .getattr("modules")?
            .set_item("keygen_sh.license_file", m)
    })?;

    m.add_class::<LicenseFile>()?;
    Ok(())
}

create_interface!(LicenseFile, KeygenRsLicenseFile);

impl LicenseFile {}

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
}