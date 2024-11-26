use pyo3::prelude::*;
use crate::utils::create_interface;
use keygen_rs::component::Component as KeygenRsComponent;
use crate::json::JsonValue;

#[pymodule(name = "component")]
pub fn component_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Hack: workaround for https://github.com/PyO3/pyo3/issues/759
    Python::with_gil(|py| {
        py.import("sys")?
            .getattr("modules")?
            .set_item("keygen_sh.component", m)
    })?;

    m.add_class::<Component>()?;
    Ok(())
}

create_interface!(Component, KeygenRsComponent);

#[pymethods]
impl Component {
    #[new]
    pub fn new(id: String, fingerprint: String, name: String) -> Self {
        Self {
            inner: KeygenRsComponent { id, fingerprint, name }
        }
    }

    #[getter]
    pub fn id(&self) -> PyResult<String> {
        Ok(self.inner.id.clone())
    }

    #[getter]
    pub fn fingerprint(&self) -> PyResult<String> {
        Ok(self.inner.fingerprint.clone())
    }

    #[getter]
    pub fn name(&self) -> PyResult<String> {
        Ok(self.inner.name.clone())
    }

    pub fn create_object(&self) -> PyResult<JsonValue> {
        let object = KeygenRsComponent::create_object(&self.inner);
        Ok(JsonValue(object))
    }
}

impl Into<KeygenRsComponent> for Component {
    fn into(self) -> KeygenRsComponent {
        KeygenRsComponent {
            id: self.inner.id,
            fingerprint: self.inner.fingerprint,
            name: self.inner.name,
        }
    }
}