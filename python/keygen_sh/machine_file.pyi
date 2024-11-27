import datetime

from keygen_sh import License
from keygen_sh.certificate import Certificate
from keygen_sh.machine import Machine

class MachineFileDataset:
    license: License
    machine: Machine
    issued: datetime.datetime
    expiry: datetime.datetime
    ttl: int


class MachineFile:
    id: str
    certificate: str
    issued: datetime.datetime
    expiry: datetime.datetime
    ttl: int

    @staticmethod
    def from_cert(key: str, content: str) -> MachineFile:
        """
        Creates a MachineFile instance from the given certificate key and content.

        :param key: The certificate key used to create the MachineFile.
        :type key: str
        :param content: The content of the certificate used to create the MachineFile.
        :type content: str
        :return: A new MachineFile instance initialized with the provided key and content.
        :rtype: MachineFile
        """
        ...

    def verify(self) -> None:
        """
        Verify the validity of the MachineFile
        """
        ...

    def decrypt(self, key: str) -> Machine:
        """
        Decrypts the contents using the provided key.

        :param key: The decryption key in string format.
        :return: A Machine instance.
        :rtype: Machine
        """
        ...

    def build_certificate(self) -> Certificate:
        """
        Builds a machine certificate.

        :return: A Certificate instance.
        :rtype: Certificate
        """
        ...
    
    

