import datetime

from keygen_sh import License
from keygen_sh.certificate import Certificate


class LicenseFileDataset:
    license: License
    issued: datetime.datetime
    expiry: datetime.datetime
    ttl: int

class LicenseFile:
    id: str
    certificate: str
    issued: datetime.datetime
    expiry: datetime.datetime
    ttl: int

    @staticmethod
    def build_from_cert(key: str, content: str) -> LicenseFile:
        """
        Build a certificate.

        :param key: key for the certificate
        :type key: str
        :param content: content for the certificate
        :type content: str
        :return: Instance of a LicenseFile
        :rtype LicenseFile
        """
        ...

    def verify(self) -> None:
        """
        Verifies the license file
        """
        ...

    def decrypt(self, key: str) -> LicenseFileDataset:
        """
        Decrypts the license file and returns a license file dataset instance.
        :return: LicenseFileDataset instance associated with this LicenseFile and the given key
        :rtype LicenseFileDataset
        """
        ...

    def build_certificate(self) -> Certificate:
        """
        Build a certificate out of the LicenseFile instance.
        :return: Certificate associated with this LicenseFile
        :rtype Certificate
        """
        ...

