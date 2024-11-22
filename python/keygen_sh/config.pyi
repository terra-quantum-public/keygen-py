from dataclasses import dataclass
from typing import Optional

@dataclass(frozen=True)
class KeygenConfig:
    api_url: str
    api_version: str
    api_prefix: str
    account: str
    product: str
    package: Optional[str] = ""
    environment: Optional[str] = None
    license_key: Optional[str] = None
    token: Optional[str] = None
    public_key: Optional[str] = None
    platform: Optional[str] = None
    user_agent: Optional[str] = None
    max_clock_drift: Optional[int] = 5
    

def set_config(config: KeygenConfig) -> None:
    """
    Set the global configuration.

    :param config: a configuration object
    """
    ...

def get_config() -> KeygenConfig:
    """
    Gets the global configuration.

    :return global configuration object
    """
    ...
