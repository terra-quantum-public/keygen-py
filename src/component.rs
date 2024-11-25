use pyo3::prelude::*;
use crate::utils::create_interface;
use keygen_rs::component::Component as KeygenRsComponent;

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

    // pub fn create_object(&self) -> serde_json::Value {
    //     let object = KeygenRsComponent::create_object(&self.inner);
    //
    //     match object {
    //         _ => object,
    //     }
    // }
}