import pytest
from config_fixture import keygen_config

@pytest.mark.asyncio
async def test_license_validate(keygen_config):
    from keygen_sh import validate

    l = await validate([], [])
    assert l.status == "ACTIVE"
    res = await l.validate([], [])
    assert res


@pytest.mark.asyncio
async def test_license_machines():
    from keygen_sh import validate

    l = await validate([], [])
    assert l.status == "ACTIVE"
    ms = await l.machines()
    assert ms
    m = await l.machine(id=ms[0].id)
    assert m
