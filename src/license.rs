use crate::date::Date;
use keygen_rs::license;
use pyo3::pyclass;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub enum SchemeCode {
    Ed25519Sign,
}

#[pyclass(dict, get_all, frozen)]
#[derive(Debug, Clone)]
pub struct License {
    pub id: String,
    pub scheme: Option<SchemeCode>,
    pub key: String,
    pub name: Option<String>,
    pub expiry: Option<Date>,
    pub status: Option<String>,
    pub policy: Option<String>,
}

impl License {
    pub(crate) fn from(keygen_license: license::License) -> License {
        License {
            expiry: keygen_license.expiry.map(Date::from),
            id: keygen_license.id,
            key: keygen_license.key,
            name: keygen_license.name,
            policy: keygen_license.policy,
            scheme: keygen_license.scheme.map(|scheme| match scheme {
                license::SchemeCode::Ed25519Sign => SchemeCode::Ed25519Sign,
            }),
            status: keygen_license.status,
        }
    }
}

impl From<SchemeCode> for license::SchemeCode {
    fn from(val: SchemeCode) -> Self {
        match val {
            SchemeCode::Ed25519Sign => license::SchemeCode::Ed25519Sign,
        }
    }
}