from typing import Optional
from .keygen_sh import *
from .errors import Error
from .keygen_sh import validate as _validate
from .keygen_sh import verify as _verify
from .license import SchemeCode, License


def verify(scheme: SchemeCode, signed_key: str) -> str:
    """
    Verify an offline key.

    :param scheme: the scheme code
    :param signed_key: the key to verify
    :return: the data in the key
    :raises: a keygen_sh.errors.Error if the key is invalid
    """

    try:
        return _verify(scheme, signed_key)
    except keygen_sh._errors.KeygenError as ex:
        raise Error.from_json(ex.args[0]) from None

async def validate(
        fingerprints: Optional[list[str]] = None, entitlements: Optional[list[str]] = None
) -> License:
    """
    Validate a license.

    :param fingerprints: optionally, the fingerprints
    :param entitlements: optionally, the entitlements
    :raises: a keygen_sh.errors.Error if the license is invalid
    """

    try:
        return await _validate(fingerprints, entitlements)
    except keygen_sh._errors.KeygenError as ex:
        raise Error.from_json(ex.args[0]) from None
