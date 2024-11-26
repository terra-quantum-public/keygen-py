import datetime
from enum import Enum, auto
from typing import Optional, List

from machine import Machine


class SchemeCode(Enum):
    Ed25519Sign = auto()

class License:
    id: str
    scheme: SchemeCode
    key: str
    name: Optional[str]
    expiry: Optional[datetime.datetime]
    status: Optional[str]
    policy: Optional[str]

    async def validate(
        self, fingerprints: Optional[list[str]] = None, entitlements: Optional[list[str]] = None
    ) -> License:
        """
        Validate a license.

        :param fingerprints: optionally, the fingerprints
        :param entitlements: optionally, the entitlements
        :raises: RuntimeError if the license is invalid
        """
    ...

    async def machine(self, id: str) -> Machine:
        """
        Get the machines associated with this license.
        :return: Machines associated with this license
        """
    ...

    async def machines(self) -> List[Machine]:
        """
        Get the machines associated with this license.
        :return: Machines associated with this license
        """
    ...

