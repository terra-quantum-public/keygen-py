import datetime
from enum import Enum, auto
from typing import Optional, List

from keygen_sh.component import Component
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

    @staticmethod
    async def activate_machine(
        license_id: str, fingerprint: str, components: Optional[list[Component]] = None
    ) -> Machine:
        """
        Activate a machine

        :param license_id: the id of the license
        :param fingerprint: the fingerprint of the machine
        :param components: optionally a list of components
        :returns: the machine instance
        """
        ...

    async def validate(
        self,
        fingerprints: Optional[list[str]] = None,
        entitlements: Optional[list[str]] = None,
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
