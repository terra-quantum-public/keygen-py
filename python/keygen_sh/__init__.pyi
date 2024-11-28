from typing import Optional

from .errors import KeygenError
from .license import SchemeCode, License

def verify(scheme: SchemeCode, signed_key: str) -> str:
    """
    Verify an offline key.

    :param scheme: the scheme code
    :param signed_key: the key to verify
    :return: the data in the key
    :raises: a keygen_sh.errors.KeygenError if the key is invalid
    """
    ...

async def validate(
    fingerprints: Optional[list[str]] = None, entitlements: Optional[list[str]] = None
) -> License:
    """
    Validate a license.

    :param fingerprints: optionally, the fingerprints
    :param entitlements: optionally, the entitlements
    :raises: a keygen_sh.errors.KeygenError if the license is invalid
    """
    ...
