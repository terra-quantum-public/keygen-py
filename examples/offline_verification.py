import os

from keygen_sh import verify
from keygen_sh.license import SchemeCode
from keygen_sh.config import KeygenConfig, set_config

set_config(
    KeygenConfig(
        api_url="https://api.keygen.sh",
        api_prefix="v1",
        api_version="v1.7",
        account=os.getenv("KEYGEN_ACCOUNT_ID"),
        product=os.getenv("KEYGEN_PRODUCT_ID"),
        license_key=os.getenv("KEYGEN_LICENSE_KEY"),
        public_key=os.getenv("KEYGEN_PUBLIC_KEY"),
    )
)

_LICENSE_KEY = os.getenv("OFFLINE_LICENSE_KEY")

try:
    data = verify(SchemeCode.Ed25519Sign, signed_key=_LICENSE_KEY)
    print(f"License is valid and contains the following data {data}")
except RuntimeError:
    print("License could not be verified.")
