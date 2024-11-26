from config_fixture import keygen_config

def test_import():
    from keygen_sh.config import set_config, KeygenConfig
    assert KeygenConfig
    assert set_config

def test_set_config(keygen_config):
    from keygen_sh.config import get_config
    config = get_config()

    assert config.api_url == "https://api.keygen.sh"
    assert config.user_agent is None

