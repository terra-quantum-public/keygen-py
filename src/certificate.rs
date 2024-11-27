use pyo3::prelude::*;
use pyo3::{pyclass, pymodule, Bound, PyResult, Python};

#[pymodule(name = "certificate")]
pub fn certificate_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Hack: workaround for https://github.com/PyO3/pyo3/issues/759
    Python::with_gil(|py| {
        py.import("sys")?
            .getattr("modules")?
            .set_item("keygen_sh.certificate", m)
    })?;

    m.add_class::<Certificate>()?;
    Ok(())
}

#[pyclass(frozen)]
#[derive(Debug, Clone)]
pub struct Certificate {
    pub enc: String,
    pub sig: String,
    pub alg: String,
}

impl Certificate {
    pub(crate) fn build(
        enc: String,
        sig: String,
        alg: String,
    ) -> Self {
        Self { enc, sig, alg }
    }
}