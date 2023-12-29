# DecentralML Python client

The python client offers an interface to access the functionalities of the DecentraML node throught scripts.

```
└── decentralml
    ├── __init__.py
    ├── assign_task.py
    ├── create_task.py
    ├── examples.py
    ├── main.py
    ├── script.py
    ├── send_task_result.py
    ├── settings.py
    ├── storage_ipfs.py
    └── utilities.py
```

There are several python scripts that correlate to create_task, assign_task, send_task_result, validate_task_result, accept_task_result, reject_task_result, list_tasks, list_task_results which allows interaction with Substrate DecentralML node and custom pallets.

The scripts correspond to the task described [Insert gitbook link here]().

The client exposes and provides methods to communicate to fund actors involved and execute several tasks on the node.

To use the client, the best method is to use the docker container following the instruction [here](https://github.com/livetreetech/DecentralML/blob/main/docker/README.md).

### Installation and local testing

> **NOTE** The docker containers are the advised method of testing. If you decide to test locally on your own machine, please be advised that some errors might occur and that some adjustments might be needed to the following commands, according to your system.

Python (version >= 3.10), pip and poetry packages are required to run the script.

- Install python https://www.datacamp.com/blog/how-to-install-python

- Install pip https://pip.pypa.io/en/stable/installation/


Then, after cloning the repository, switch to the decentraml/substrate-client-decentralml directory.

```bash
cd DecentralML/substrate-client-decentralml/
```

Create a python environment:
```bash
python -m venv .venv
```

Activate the environment:
```bash
source .venv/bin/activate
```

After that, install poetry with:

```bash
pip install poetry
```

`Poetry` is a python packaging and dependencies management system that facilitate the installation of python code as packages. More info can be found at https://python-poetry.org/docs/.

Then install the client with:

```bash
poetry install
```

After this, making sure that a node is running, either with a docker or locally on the machine, you can run the examples with the following command:

```bash
python -m decentralml.examples
```

Or you can launch the main menu for each role:

```bash
python -m decentralml.main
```

> **NOTE** If you run the client locally (NOT in the docker), make sure that you place your shell in the root directory of the project. This is required for the assets folder to be found at `substrate-client-decentralml/assets`

Any other script can be run as part of the module `decentralml` with:

```bash
python -m decentralml.xxxx
```

where `xxxx` is one of the script in the module.