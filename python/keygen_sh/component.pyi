from dataclasses import dataclass
from typing import Dict


@dataclass(frozen=True)
class Component:
    id: str
    fingerprint: str
    name: str

    def create_object(self) -> Dict:
        """
        Creates dictionary representation of this Component instance.
        """
        ...
