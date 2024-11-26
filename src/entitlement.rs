use chrono::{DateTime, Utc};
use pyo3::prelude::{PyAnyMethods, PyModule, PyModuleMethods};
use pyo3::{pyclass, pymodule, Bound, PyResult, Python};

#[pymodule(name = "entitlement")]
pub fn entitlement_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Hack: workaround for https://github.com/PyO3/pyo3/issues/759
    Python::with_gil(|py| {
        py.import("sys")?
            .getattr("modules")?
            .set_item("keygen_sh.entitlement", m)
    })?;

    m.add_class::<Entitlement>()?;
    Ok(())
}

#[pyclass(frozen)]
#[derive(Debug, Clone)]
pub struct Entitlement {
    id: String,
    name: Option<String>,
    code: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl Entitlement {
    pub(crate) fn build(
        id: String,
        name: Option<String>,
        code: String,
        created: DateTime<Utc>,
        updated: DateTime<Utc>,
    ) -> Self {
        Self { id, name, code, created, updated }
    }
}