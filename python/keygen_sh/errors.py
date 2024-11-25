from dataclasses import dataclass
import json
from typing import Type


class Error(Exception):
    def __init__(self, error_message: str, details: str = None):
        super().__init__(error_message)
        self.error_message = error_message

    @classmethod
    def from_json(cls, error_json: str) -> "Error":
        error_dict = json.loads(error_json)
        error_type = error_dict.get("type")
        error_class = cls.get_error_class(error_type)
        if isinstance(error_dict.get('details'), str):
            return error_class()

        return error_class(**error_dict.get('details'))

    @staticmethod
    def get_error_class(error_type: str) -> Type["Error"]:
        error_classes = {
            "UnexpectedError": UnexpectedError,
            "InvalidUrl": InvalidUrl,
            "SystemClockUnsynced": SystemClockUnsynced,
            "DecryptionError": DecryptionError,
            "HttpClient": HttpClient,
            "UrlParse": UrlParse,
            "JsonError": JsonError,
            "InvalidHeader": InvalidHeader,
            "UrlEncode": UrlEncode,
            "RateLimitExceeded": RateLimitExceeded,
            "LicenseKeyMissing": LicenseKeyMissing,
            "LicenseSchemeMissing": LicenseSchemeMissing,
            "LicenseSchemeNotSupported": LicenseSchemeNotSupported,
            "LicenseNotSigned": LicenseNotSigned,
            "LicenseKeyNotGenuine": LicenseKeyNotGenuine,
            "PublicKeyMissing": PublicKeyMissing,
            "PublicKeyInvalid": PublicKeyInvalid,
            "LicenseSchemeUnsupported": LicenseSchemeUnsupported,
            "CerificateFileInvalid": CerificateFileInvalid,
            "CertificateFileNotGenuine": CertificateFileNotGenuine,
            "CertificateFileNotSupported": CertificateFileNotSupported,
            "CerificateFileExpired": CerificateFileExpired,
            "LicenseFileInvalid": LicenseFileInvalid,
            "LicenseFileNotGenuine": LicenseFileNotGenuine,
            "LicenseFileNotSupported": LicenseFileNotSupported,
            "LicenseFileNotEncrypted": LicenseFileNotEncrypted,
            "LicenseFileExpired": LicenseFileExpired,
            "MachineFileInvalid": MachineFileInvalid,
            "MachineFileNotGenuine": MachineFileNotGenuine,
            "MachineFileNotSupported": MachineFileNotSupported,
            "MachineFileExpired": MachineFileExpired,
            "KeygenApiError": KeygenApiError,
            "TokenNotAllowed": TokenNotAllowed,
            "TokenFormatInvalid": TokenFormatInvalid,
            "TokenInvalid": TokenInvalid,
            "TokenExpired": TokenExpired,
            "LicenseSuspended": LicenseSuspended,
            "LicenseExpired": LicenseExpired,
            "LicenseNotAllowed": LicenseNotAllowed,
            "LicenseNotActivated": LicenseNotActivated,
            "LicenseKeyInvalid": LicenseKeyInvalid,
            "LicenseTokenInvalid": LicenseTokenInvalid,
            "LicenseTooManyMachines": LicenseTooManyMachines,
            "LicenseTooManyCores": LicenseTooManyCores,
            "LicenseTooManyProcesses": LicenseTooManyProcesses,
            "MachineAlreadyActivated": MachineAlreadyActivated,
            "MachineLimitExceeded": MachineLimitExceeded,
            "MachineNotFound": MachineNotFound,
            "ProcessLimitExceeded": ProcessLimitExceeded,
            "ProcessNotFound": ProcessNotFound,
            "ComponentConflict": ComponentConflict,
            "ComponentAlreadyActivated": ComponentAlreadyActivated,
            "ComponentNotActivated": ComponentNotActivated,
            "EnvironmentError": EnvironmentError,
            "HeartbeatDead": HeartbeatDead,
            "HeartbeatPingFailed": HeartbeatPingFailed,
            "HeartbeatRequired": HeartbeatRequired,
            "ValidationFingerprintMissing": ValidationFingerprintMissing,
            "ValidationComponentsMissing": ValidationComponentsMissing,
            "ValidationProductMissing": ValidationProductMissing,
            "NotFound": NotFound,
        }
        return error_classes.get(error_type, Error)


@dataclass
class UnexpectedError(Error):
    details: str

    def __init__(self, details: str):
        super().__init__(f"Unexpected error: {details}")
        self.details = details


@dataclass
class InvalidUrl(Error):

    def __init__(self):
        super().__init__("Invalid URL")


@dataclass
class SystemClockUnsynced(Error):

    def __init__(self):
        super().__init__("System clock is out of sync")


@dataclass
class DecryptionError(Error):
    details: str

    def __init__(self, details: str):
        super().__init__(f"Decryption error: {details}")
        self.details = details


@dataclass
class HttpClient(Error):
    details: str

    def __init__(self, details: str):
        super().__init__(f"HTTP client error: {details}")
        self.details = details


@dataclass
class UrlParse(Error):
    details: str

    def __init__(self, details: str):
        super().__init__(f"URL parse error: {details}")
        self.details = details


@dataclass
class JsonError(Error):
    details: str

    def __init__(self, details: str):
        super().__init__(f"JSON error: {details}")
        self.details = details


@dataclass
class InvalidHeader(Error):
    details: str

    def __init__(self, details: str):
        super().__init__(f"Invalid header value: {details}")
        self.details = details


@dataclass
class UrlEncode(Error):
    details: str

    def __init__(self, details: str):
        super().__init__(f"URL encoding error: {details}")
        self.details = details


@dataclass
class RateLimitExceeded(Error):
    window: str
    count: int
    limit: int
    remaining: int
    reset: int
    retry_after: int

    def __init__(self, window: str, count: int, limit: int, remaining: int, reset: int, retry_after: int):
        super().__init__(
            f"Rate limit exceeded: window={window}, count={count}, limit={limit}, remaining={remaining}, reset={reset}, retry_after={retry_after}")
        self.window = window
        self.count = count
        self.limit = limit
        self.remaining = remaining
        self.reset = reset
        self.retry_after = retry_after


@dataclass
class LicenseKeyMissing(Error):

    def __init__(self):
        super().__init__("License key is missing")


@dataclass
class LicenseSchemeMissing(Error):

    def __init__(self):
        super().__init__("License scheme is missing")


@dataclass
class LicenseSchemeNotSupported(Error):

    def __init__(self):
        super().__init__("License scheme is not supported")


@dataclass
class LicenseNotSigned(Error):

    def __init__(self):
        super().__init__("License is not signed")


@dataclass
class LicenseKeyNotGenuine(Error):

    def __init__(self):
        super().__init__("License key is not genuine")


@dataclass
class PublicKeyMissing(Error):

    def __init__(self):
        super().__init__("Public key is missing")


@dataclass
class PublicKeyInvalid(Error):

    def __init__(self):
        super().__init__("Public key is invalid")


@dataclass
class LicenseSchemeUnsupported(Error):

    def __init__(self):
        super().__init__("License scheme unsupported")


@dataclass
class CerificateFileInvalid(Error):
    details: str

    def __init__(self, details: str):
        super().__init__(details)
        self.details = details


@dataclass
class CertificateFileNotGenuine(Error):
    details: str

    def __init__(self, details: str):
        super().__init__(details)
        self.details = details


@dataclass
class CertificateFileNotSupported(Error):
    details: str

    def __init__(self, details: str):
        super().__init__(details)
        self.details = details


@dataclass
class CerificateFileExpired(Error):

    def __init__(self):
        super().__init__("Cerificate file expired")


@dataclass
class LicenseFileInvalid(Error):
    details: str

    def __init__(self, details: str):
        super().__init__(f"License file invalid: {details}")
        self.details = details


@dataclass
class LicenseFileNotGenuine(Error):
    details: str

    def __init__(self, details: str):
        super().__init__(f"License file not genuine: {details}")
        self.details = details


@dataclass
class LicenseFileNotSupported(Error):
    details: str

    def __init__(self, details: str):
        super().__init__(f"License file not supported: {details}")
        self.details = details


@dataclass
class LicenseFileNotEncrypted(Error):

    def __init__(self):
        super().__init__("License file not encrypted")


@dataclass
class LicenseFileExpired(Error):
    dataset: dict  # Assuming LicenseFileDataset is a dictionary

    def __init__(self, dataset: dict):
        super().__init__("License file expired")
        self.dataset = dataset


@dataclass
class MachineFileInvalid(Error):
    details: str

    def __init__(self, details: str):
        super().__init__("Machine file invalid")
        self.details = details


@dataclass
class MachineFileNotGenuine(Error):
    details: str

    def __init__(self, details: str):
        super().__init__("Machine file not genuine")
        self.details = details


@dataclass
class MachineFileNotSupported(Error):
    details: str

    def __init__(self, details: str):
        super().__init__("Machine file not supported")
        self.details = details


@dataclass
class MachineFileExpired(Error):
    dataset: dict  # Assuming MachineFileDataset is a dictionary

    def __init__(self, dataset: dict):
        super().__init__("License file expired")
        self.dataset = dataset


@dataclass
class KeygenApiError(Error):
    code: str
    detail: str
    body: dict  # Assuming serde_json::Value is a dictionary

    def __init__(self, code: str, detail: str, body: dict):
        super().__init__(f"API error: {detail}")
        self.code = code
        self.detail = detail
        self.body = body


@dataclass
class TokenNotAllowed(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Token not allowed")
        self.code = code
        self.detail = detail


@dataclass
class TokenFormatInvalid(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Token format invalid")
        self.code = code
        self.detail = detail


@dataclass
class TokenInvalid(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Token invalid")
        self.code = code
        self.detail = detail


@dataclass
class TokenExpired(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Token expired")
        self.code = code
        self.detail = detail


@dataclass
class LicenseSuspended(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("License suspended")
        self.code = code
        self.detail = detail


@dataclass
class LicenseExpired(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("License expired")
        self.code = code
        self.detail = detail


@dataclass
class LicenseNotAllowed(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("License not allowed")
        self.code = code
        self.detail = detail


@dataclass
class LicenseNotActivated(Error):
    code: str
    detail: str
    license: dict  # Assuming License is a dictionary

    def __init__(self, code: str, detail: str, license: dict):
        super().__init__("License not activated")
        self.code = code
        self.detail = detail
        self.license = license


@dataclass
class LicenseKeyInvalid(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("License key invalid")
        self.code = code
        self.detail = detail


@dataclass
class LicenseTokenInvalid(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("License token invalid")
        self.code = code
        self.detail = detail


@dataclass
class LicenseTooManyMachines(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("License has too many machines")
        self.code = code
        self.detail = detail


@dataclass
class LicenseTooManyCores(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("License has too many cores")
        self.code = code
        self.detail = detail


@dataclass
class LicenseTooManyProcesses(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("License has too many processes")
        self.code = code
        self.detail = detail


@dataclass
class MachineAlreadyActivated(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Machine already activated")
        self.code = code
        self.detail = detail


@dataclass
class MachineLimitExceeded(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Machine limit exceeded")
        self.code = code
        self.detail = detail


@dataclass
class MachineNotFound(Error):

    def __init__(self):
        super().__init__("Machine no longer exists")


@dataclass
class ProcessLimitExceeded(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Process limit exceeded")
        self.code = code
        self.detail = detail


@dataclass
class ProcessNotFound(Error):

    def __init__(self):
        super().__init__("Process no longer exists")


@dataclass
class ComponentConflict(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Component conflict")
        self.code = code
        self.detail = detail


@dataclass
class ComponentAlreadyActivated(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Component already activated")
        self.code = code
        self.detail = detail


@dataclass
class ComponentNotActivated(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Component is not activated")
        self.code = code
        self.detail = detail


@dataclass
class EnvironmentError(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Environment error")
        self.code = code
        self.detail = detail


@dataclass
class HeartbeatDead(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Heartbeat dead")
        self.code = code
        self.detail = detail


@dataclass
class HeartbeatPingFailed(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Heartbeat ping failed")
        self.code = code
        self.detail = detail


@dataclass
class HeartbeatRequired(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Heartbeat is required")
        self.code = code
        self.detail = detail


@dataclass
class ValidationFingerprintMissing(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Validation fingerprint scope is missing")
        self.code = code
        self.detail = detail


@dataclass
class ValidationComponentsMissing(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Validation components scope is missing")
        self.code = code
        self.detail = detail


@dataclass
class ValidationProductMissing(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Validation product scope is missing")
        self.code = code
        self.detail = detail


@dataclass
class NotFound(Error):
    code: str
    detail: str

    def __init__(self, code: str, detail: str):
        super().__init__("Not found")
        self.code = code
        self.detail = detail
