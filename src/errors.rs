use keygen_rs::errors::Error;
use pyo3::{create_exception, pymodule, Bound, PyErr, PyResult};
use pyo3::exceptions::PyException;
use pyo3::prelude::{PyModule, PyModuleMethods};
use serde_json::json;

create_exception!(errors_module, KeygenError, PyException);

#[pymodule]
#[pyo3(name = "_errors")]
pub fn errors_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("KeygenError", m.py().get_type::<KeygenError>())?;
    Ok(())
}

impl KeygenError {
    pub(crate) fn from_error(value: Error) -> PyErr {
        match value {
            Error::UnexpectedError(detail) => {
                let info = json!({ "type": "UnexpectedError", "details": detail });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::InvalidUrl => {
                let info = json!({ "type": "InvalidUrl", "details": "Invalid URL" });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::SystemClockUnsynced => {
                let info = json!({ "type": "SystemClockUnsynced", "details": "System clock unsynced" });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::DecryptionError(detail) => {
                let info = json!({ "type": "DecryptionError", "details": detail });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::RateLimitExceeded { window, count, limit, remaining, reset, retry_after } => {
                let info = json!({ "type": "RateLimitExceeded", "details": { "window": window, "count" : count, "limit": limit, "remaining": remaining, "reset": reset, "retry_after": retry_after } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::LicenseKeyMissing => {
                let info = json!({ "type": "LicenseKeyMissing", "details": "License key missing" });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::LicenseSchemeMissing => {
                let info = json!({ "type": "LicenseSchemeMissing", "details": "License scheme missing" });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::LicenseSchemeNotSupported => {
                let info = json!({ "type": "LicenseSchemeNotSupported", "details": "License scheme not supported" });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::LicenseNotSigned => {
                let info = json!({ "type": "LicenseNotSigned", "details": "License not signed" });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::LicenseKeyNotGenuine => {
                let info = json!({ "type": "LicenseKeyNotGenuine", "details": "License key not genuine" });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::PublicKeyMissing => {
                let info = json!({ "type": "PublicKeyMissing", "details": "Public key missing" });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::PublicKeyInvalid => {
                let info = json!({ "type": "PublicKeyInvalid", "details": "Public key invalid" });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::LicenseSchemeUnsupported => {
                let info = json!({ "type": "LicenseSchemeUnsupported", "details": "License scheme unsupported" });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::CerificateFileInvalid(detail) => {
                let info = json!({ "type": "CerificateFileInvalid", "details": detail });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::CertificateFileNotGenuine(detail) => {
                let info = json!({ "type": "CertificateFileNotGenuine", "details": detail });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::CertificateFileNotSupported(detail) => {
                let info = json!({ "type": "CertificateFileNotSupported", "details": detail });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::CerificateFileExpired => {
                let info = json!({ "type": "CerificateFileExpired", "details": "Certificate file expired" });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::LicenseFileInvalid(detail) => {
                let info = json!({ "type": "LicenseFileInvalid", "details": detail });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::LicenseFileNotGenuine(detail) => {
                let info = json!({ "type": "LicenseFileNotGenuine", "details": detail });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::LicenseFileNotSupported(detail) => {
                let info = json!({ "type": "LicenseFileNotSupported", "details": detail });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::LicenseFileNotEncrypted => {
                let info = json!({ "type": "LicenseFileNotEncrypted", "details": "License file not encrypted" });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::LicenseFileExpired(detail) => {
                let info = json!({ "type": "LicenseFileExpired", "details": detail });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::MachineFileInvalid(detail) => {
                let info = json!({ "type": "MachineFileInvalid", "details": detail });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::MachineFileNotGenuine(detail) => {
                let info = json!({ "type": "MachineFileNotGenuine", "details": detail });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::MachineFileNotSupported(detail) => {
                let info = json!({ "type": "MachineFileNotSupported", "details": detail });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::MachineFileExpired(detail) => {
                let info = json!({ "type": "MachineFileExpired", "details": detail });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::KeygenApiError {code, detail, body} => {
                let info = json!({ "type": "KeygenApiError", "details": {
                    "code": code,
                    "detail": detail,
                    "body": body
                } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::TokenNotAllowed { detail, code } => {
                let info = json!({ "type": "TokenNotAllowed", "details": { "detail": detail, "code": code } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::TokenFormatInvalid {code, detail} => {
                let info = json!({ "type": "TokenFormatInvalid", "details": { "detail": detail, "code": code} });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::TokenInvalid {code, detail} => {
                let info = json!({ "type": "TokenInvalid", "details": {"detail": detail, "code": code} });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::TokenExpired {code, detail} => {
                let info = json!({ "type": "TokenExpired", "details": { "detail": detail, "code": code } });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::LicenseSuspended { code, detail } => {
                let info = json!({ "type": "LicenseSuspended", "details": { "detail": detail, "code": code  } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::LicenseExpired { detail, code } => {
                let info = json!({ "type": "LicenseExpired", "details": { "detail": detail, "code": code  } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::LicenseNotAllowed { detail, code } => {
                let info = json!({ "type": "LicenseNotAllowed", "details": { "detail": detail, "code": code } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::LicenseNotActivated { code, detail, license } => {
                let info = json!({ "type": "LicenseNotActivated", "details": { "detail": detail, "code": code, "license": license } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::LicenseKeyInvalid { detail, code } => {
                let info = json!({ "type": "LicenseKeyInvalid", "details": { "detail": detail, "code": code  } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::LicenseTokenInvalid { detail, code } => {
                let info = json!({ "type": "LicenseTokenInvalid", "details": { "detail": detail, "code": code  } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::LicenseTooManyMachines { detail, code } => {
                let info = json!({ "type": "LicenseTooManyMachines", "details": { "detail": detail, "code": code  } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::LicenseTooManyCores { code, detail } => {
                let info = json!({ "type": "LicenseTooManyCores", "details": { "detail": detail, "code": code  }});
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::LicenseTooManyProcesses { detail, code } => {
                let info = json!({ "type": "LicenseTooManyProcesses", "details": { "detail": detail, "code": code  } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::MachineAlreadyActivated { detail, code } => {
                let info = json!({ "type": "MachineAlreadyActivated", "details": { "detail": detail, "code": code  } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::MachineLimitExceeded { detail, code} => {
                let info = json!({ "type": "MachineLimitExceeded", "details": { "detail": detail, "code": code  } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::MachineNotFound => {
                let info = json!({ "type": "MachineNotFound", "details": "Machine not found" });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::ProcessLimitExceeded { detail, code } => {
                let info = json!({ "type": "ProcessLimitExceeded", "details": {"detail": detail, "code": code } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::ProcessNotFound => {
                let info = json!({ "type": "ProcessNotFound", "details": "Process not found" });
                KeygenError::new_err(serde_json::to_string(&info).unwrap_or_else(|_| "Serialization error".to_string()))
            }
            Error::ComponentConflict { code, detail } => {
                let info = json!({ "type": "ComponentConflict", "details": { "detail": detail, "code": code } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::ComponentAlreadyActivated { code, detail } => {
                let info = json!({ "type": "ComponentAlreadyActivated", "details": { "detail": detail, "code": code  } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::ComponentNotActivated { code, detail } => {
                let info = json!({ "type": "ComponentNotActivated", "details": { "detail": detail, "code": code  } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::EnvironmentError { code, detail } => {
                let info = json!({ "type": "EnvironmentError", "details": { "detail": detail, "code": code } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::HeartbeatDead { code, detail } => {
                let info = json!({ "type": "HeartbeatDead", "details": { "detail": detail, "code": code  } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::HeartbeatPingFailed { code, detail } => {
                let info = json!({ "type": "HeartbeatPingFailed", "details": { "detail": detail, "code": code  } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::HeartbeatRequired { code, detail } => {
                let info = json!({ "type": "HeartbeatRequired", "details": { "detail": detail, "code": code  } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::ValidationFingerprintMissing { code, detail } => {
                let info = json!({ "type": "ValidationFingerprintMissing", "details": { "detail": detail, "code": code  } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            Error::ValidationComponentsMissing { code, detail } => {
                let info = json!({ "type": "ValidationComponentsMissing", "details": { "detail": detail, "code": code } });
                serde_json::to_string(&info)
                    .map(KeygenError::new_err)
                    .unwrap_or_else(|_| KeygenError::new_err("Serialization error"))
            }
            _ => {
                KeygenError::new_err("Unknown error")
            }
        }
    }
}
