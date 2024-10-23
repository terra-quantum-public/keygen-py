use keygen_rs::config;
use pyo3::{pyclass, pymethods};


#[pyclass]
#[derive(Debug, Clone)]
pub struct KeygenConfig {
    inner: config::KeygenConfig,
}

#[pymethods]
impl KeygenConfig {
    #[pyo3(signature = (api_url, api_version, api_prefix, account, product, package=None, environment=None, license_key=None, token=None, public_key=None, platform=None, user_agent=None, max_clock_drift=5)
    )]
    #[new]
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
            inner: config::KeygenConfig {
                api_url,
                api_version,
                api_prefix,
                account,
                product,
                package: package.or(Some("".to_string())).unwrap(),
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
}

impl Into<config::KeygenConfig> for KeygenConfig {
    fn into(self) -> config::KeygenConfig {
        self.inner
    }
}