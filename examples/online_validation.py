import asyncio
import os
import uuid

from keygen_sh import validate
from keygen_sh.config import set_config, KeygenConfig

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


async def amain():
    fingerprint = str(uuid.getnode())

    try:
        license = await validate(fingerprints=[fingerprint])
        # or if the license is not bound to a specific machine identity
        license = await validate()

        print(f"License {license.id}:{license.name} is valid")
    except RuntimeError:
        print("License is not valid")


if __name__ == "__main__":
    asyncio.run(amain())
