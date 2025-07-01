from os import getenv


def get_env(env_name: str, fallback: str = "") -> str:
    """Get env from the environment"""
    env = getenv(env_name)
    if not env:
        return fallback
    return env
