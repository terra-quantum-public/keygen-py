use keygen_rs;
use pyo3::prelude::*;
use pyo3::{pyclass, pyfunction, pymethods, pymodule, wrap_pyfunction, Bound, PyResult};


#[pymodule(name = "config")]
pub fn config_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Hack: workaround for https://github.com/PyO3/pyo3/issues/759
    Python::with_gil(|py| {
        py.import("sys")?
            .getattr("modules")?
            .set_item("keygen_sh.config", m)
    })?;

    m.add_function(wrap_pyfunction!(set_config, m)?)?;
    m.add_function(wrap_pyfunction!(get_config, m)?)?;
    m.add_class::<KeygenConfig>()?;
    Ok(())
}

#[pyfunction]
pub fn set_config(config: KeygenConfig) -> PyResult<()> {
    keygen_rs::config::set_config(config.into());
    Ok(())
}

#[pyfunction]
pub fn get_config() -> PyResult<KeygenConfig> {
    Ok(KeygenConfig {
        inner: keygen_rs::config::get_config(),
    })
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct KeygenConfig {
    inner: keygen_rs::config::KeygenConfig,
}

#[pymethods]
impl KeygenConfig {
    #[pyo3(signature = (api_url, api_version, api_prefix, account, product, package=None, environment=None, license_key=None, token=None, public_key=None, platform=None, user_agent=None, max_clock_drift=5)
    )]
    #[new]
    #[allow(clippy::too_many_arguments)]
    pub fn new(api_url: String,
               api_version: String,
               api_prefix: String,
               account: String,
               product: String,
               package: Option<String>,
               environment: Option<String>,
               license_key: Option<String>,
               token: Option<String>,
               public_key: Option<String>,
               platform: Option<String>,
               user_agent: Option<String>,
               max_clock_drift: Option<i64>,
    ) -> Self {
        KeygenConfig {
            inner: keygen_rs::config::KeygenConfig {
                api_url,
                api_version,
                api_prefix,
                account,
                product,
                package: package.unwrap_or("".to_string()),
                environment,
                license_key,
                token,
                public_key,
                platform,
                user_agent,
                max_clock_drift,
            }
        }
    }

    #[getter]
    pub fn api_url(&self) -> PyResult<String> {
        Ok(self.inner.api_url.clone())
    }

    #[getter]
    pub fn api_version(&self) -> PyResult<String> {
        Ok(self.inner.api_version.clone())
    }

    #[getter]
    pub fn api_prefix(&self) -> PyResult<String> {
        Ok(self.inner.api_prefix.clone())
    }

    #[getter]
    pub fn account(&self) -> PyResult<String> {
        Ok(self.inner.account.clone())
    }

    #[getter]
    pub fn product(&self) -> PyResult<String> {
        Ok(self.inner.product.clone())
    }

    #[getter]
    pub fn package(&self) -> PyResult<String> {
        Ok(self.inner.package.clone())
    }

    #[getter]
    pub fn environment(&self) -> PyResult<Option<String>> {
        Ok(self.inner.environment.clone())
    }

    #[getter]
    pub fn license_key(&self) -> PyResult<Option<String>> {
        Ok(self.inner.license_key.clone())
    }

    #[getter]
    pub fn token(&self) -> PyResult<Option<String>> {
        Ok(self.inner.token.clone())
    }

    #[getter]
    pub fn public_key(&self) -> PyResult<Option<String>> {
        Ok(self.inner.public_key.clone())
    }

    #[getter]
    pub fn platform(&self) -> PyResult<Option<String>> {
        Ok(self.inner.platform.clone())
    }

    #[getter]
    pub fn user_agent(&self) -> PyResult<Option<String>> {
        Ok(self.inner.user_agent.clone())
    }

    #[getter]
    pub fn max_clock_drift(&self) -> PyResult<Option<i64>> {
        Ok(self.inner.max_clock_drift)
    }
}

impl From<KeygenConfig> for keygen_rs::config::KeygenConfig {
    fn from(val: KeygenConfig) -> Self {
        val.inner
    }
}