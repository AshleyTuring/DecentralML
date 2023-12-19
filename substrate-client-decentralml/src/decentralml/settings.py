import os

SOCKET_URL = None

ASSETS_FOLDER = None
REMOTE_FOLDER = None

IPFS_END_POINT = None
IPFS_API_KEY = None
IPFS_API_SECRET = None
EMULATE_IPFS = None
DECENTRALML_ROLE = None

try:
    SOCKET_URL = os.environ["SOCKET_URL"]
except KeyError:
    SOCKET_URL = "ws://127.0.0.1:9944"

try:
    ASSETS_FOLDER = os.environ["ASSETS_FOLDER"]
except KeyError:
    working_directory = os.path.expanduser("~")
    ASSETS_FOLDER = os.path.join(working_directory, 'decentralml', 'assets')

try:
    REMOTE_FOLDER = os.environ["REMOTE_FOLDER"]
except KeyError:
    working_directory = os.path.expanduser("~")
    REMOTE_FOLDER = os.path.join(working_directory, 'decentralml', 'remote')

try:
    IPFS_END_POINT = os.environ["IPFS_END_POINT"]
except KeyError:
    IPFS_END_POINT = "https://ipfs.infura.io:5001"

try:
    IPFS_API_KEY = os.environ["IPFS_API_KEY"]
except KeyError:
    IPFS_API_KEY = "#######"

try:
    IPFS_API_SECRET = os.environ["IPFS_API_SECRET"]
except KeyError:
    IPFS_API_SECRET = "#######"

try:
    EMULATE_IPFS = bool(os.environ["EMULATE_IPFS"])
except KeyError:
    EMULATE_IPFS = True

try:
    DECENTRALML_ROLE = os.environ["DECENTRALML_ROLE"]
except:
    DECENTRALML_ROLE = "DECENTRALML_ROLE"

def load_user_settings():
    pass

def update_user_settings():
    pass