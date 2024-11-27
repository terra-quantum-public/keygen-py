import datetime
from typing import Optional

class Entitlement:
    id: str
    name: Optional[str]
    code: str
    created: datetime.datetime
    updated: datetime.datetime