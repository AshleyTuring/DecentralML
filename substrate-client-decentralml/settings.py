import os

try:
    SOCKET_URL = os.environ["SOCKET_URL"]
except KeyError:
    SOCKET_URL = "ws://127.0.0.1:9944"

try:
    ASSETS_FOLDER = os.environ["ASSETS_FOLDER"]
except KeyError:
    working_directory = os.getcwd()
    ASSETS_FOLDER = os.path.join(working_directory, 'substrate-client-decentralml', 'assets')

