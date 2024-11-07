from enum import Enum, auto

from typing import Optional

class KeygenConfig:
    def __init__(
        self,
        api_url: str,
        api_version: str,
        api_prefix: str,
        account: str,
        product: str,
        package: Optional[str] = "",
        environment: Optional[str] = None,
        license_key: Optional[str] = None,
        token: Optional[str] = None,
        public_key: Optional[str] = None,
        platform: Optional[str] = None,
        user_agent: Optional[str] = None,
        max_clock_drift: Optional[int] = 5,
    ): ...

class SchemeCode(Enum):
    Ed25519Sign = auto()

class License:
    id: str
    scheme: SchemeCode
    key: str
    name: Optional[str]
    expiry: Optional[str]
    status: Optional[str]
    policy: Optional[str]

def verify(scheme: SchemeCode, signed_key: str) -> str:
    """
    Verify an offline key.

    :param scheme: the scheme code
    :param signed_key: the key to verify
    :return: the data in the key
    :raises: RuntimeError if the key is invalid
    """
    ...

async def validate(
    fingerprints: Optional[list[str]] = None, entitlements: Optional[list[str]] = None
) -> License:
    """
    Validate a license.

    :param fingerprints: optionally, the fingerprints
    :param entitlements: optionally, the entitlements
    :raises: RuntimeError if the license is invalid
    """
    ...

def set_config(config: KeygenConfig) -> None:
    """
    Set the global configuration.

    :param config: a configuration object
    """
    ...
