# Unofficial Keygen Python SDK


The `keygen-py` package allows Python programs to license using the [keygen.sh](https://keygen.sh) service.
This is a wrapper around the rust package [keygen-rs](https://github.com/ahonn/keygen-rs) (so a lot of kudos there) to provide python bindings. 

## Installing

Add this to your `pyproject.toml`:

```toml
[dependencies]
keygen-py = "0.0.1.dev1"
```

## Config

### KeygenConfig

Use `KeygenConfig` to configure the SDK globally. You should set this before making any API calls.

```python
from keygen_sh import set_config, KeygenConfig

set_config(KeygenConfig(
    api_url="https://api.keygen.sh",
    api_prefix="v1",
    api_version="v1.7",
    account="YOUR_KEYGEN_ACCOUNT_ID",
    product="YOUR_KEYGEN_PRODUCT_ID",
    license_key="A_KEYGEN_LICENSE_KEY",
    public_key="YOUR_KEYGEN_PUBLIC_KEY"
))
```

## Usage

### Validate a License

To validate a license, configure `KeygenConfig` with your Keygen account details. Then call the `validate` function with a device fingerprint
(you can use [py-machineid](https://github.com/keygen-sh/py-machineid/tree/master) for this) or keep it empty depending on your policy:

```python
import asyncio
from keygen_sh import validate

async def amain():
    data = await validate(["YOUR_DEVICE_FINGERPRINT"], [])
    
    # License
    print(data.id, data.name, data.key, data.expiry)

if __name__ == '__main__':
    asyncio.run(amain())
```

### Offline License Key Verification

To verify a signed license key offline:

```python
from keygen_sh import verify, SchemeCode

data = verify(SchemeCode.Ed25519Sign, "A_KEYGEN_LICENSE_KEY")

# data encoded
print(data)
```


## Examples

For more detailed examples, please refer to the `examples` directory in the repository.

## Testing

When implementing a testing strategy for your licensing integration, we recommend mocking the Keygen API responses. This is especially important for CI/CD environments to prevent unnecessary load on Keygen's servers and to stay within your account's daily request limits.

## Inspired by
- [keygen-rs](https://github.com/ahonn/keygen-rs)
- [keygen-go](https://github.com/keygen-sh/keygen-go)

## License

This project is licensed under the MIT License.