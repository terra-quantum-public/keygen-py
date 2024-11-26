import os

import pytest

from keygen_sh.config import set_config, KeygenConfig


@pytest.fixture
def keygen_config():
    account = os.environ.get("KEYGEN_ACCOUNT_ID")
    product = os.environ.get("KEYGEN_PRODUCT")
    license_key = os.environ.get("KEYGEN_LICENSE_KEY")
    public_key = os.environ.get("KEYGEN_PUBLIC_KEY")

    # Setup code
    set_config(KeygenConfig(
        api_url="https://api.keygen.sh",
        api_prefix="v1",
        api_version="v1.7",
        account=account,
        product=product,
        license_key=license_key,
        public_key=public_key
    ))

    yield None

    # Teardown code (this will execute after the test using this fixture is done)
    # Cleanup actions if necessary
