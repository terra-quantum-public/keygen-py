use pyo3::prelude::*;
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

#[pyclass(frozen)]
#[derive(Debug, Clone)]
pub struct Component {
    inner: KeygenRsComponent,
}

#[pymethods]
impl Component {
    #[new]
    fn new(id: String, fingerprint: String, name: String) -> Self {
        Self {
            inner: KeygenRsComponent { id, fingerprint, name }
        }
    }

    #[getter]
    fn id(&self) -> PyResult<String> {
        Ok(self.inner.id.clone())
    }

    #[getter]
    fn fingerprint(&self) -> PyResult<String> {
        Ok(self.inner.fingerprint.clone())
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.inner.name.clone())
    }

    fn create_object(&self) -> PyResult<JsonValue> {
        let object = KeygenRsComponent::create_object(&self.inner);
        Ok(JsonValue(object))
    }
}

impl From<Component> for KeygenRsComponent {
    fn from(c: Component) -> KeygenRsComponent {
        c.inner
    }
}