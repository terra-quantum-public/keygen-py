from typing import Optional

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
    created: str
    updated: str
