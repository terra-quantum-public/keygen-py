import datetime
from typing import Optional, List

from keygen_sh.machine_file import MachineFile

class Machine:
    id: str
    fingerprint: str
    name: Optional[str]
    platform: Optional[str]
    hostname: Optional[str]
    cores: Optional[int]
    require_heartbeat: bool
    heartbeat_status: str
    heartbeat_duration: Optional[int]
    created: datetime.datetime
    updated: datetime.datetime

    async def deactivate(self) -> None:
        """
        Deactivate given machine.
        """
        ...

    async def checkout(self, ttl: Optional[int], include: Optional[List[str]]) -> MachineFile:
        """
        Checkout given machine.
        """
        ...

    async def ping(self) -> Machine:
        """
        Ping given machine.
        """
        ...