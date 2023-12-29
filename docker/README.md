# Running DecentralML in Docker

## Install docker

Install docker following the instructions for your system at: 

https://docs.docker.com/engine/install/

Make sure you also have installed Docker Compose:

https://docs.docker.com/compose/install/

## Build docker images

The build scripts work on Unix system, which means any Linux distribution, MacOS system or Windows Subsystem for Linux if on Windows.

- Build the docker image for the node of DecentralML

    ```./build_node.sh```
    
- Build the docker image for the python client of DecentralML

    ```./build_client.sh```

If you are on a non Linux distribution, you can just use the commands in each file in a terminal, individually. Make sure to set the correct environmental variables.

## Requirements

Before running the docker containers, make sure that:

- You have a folder and subfolder called `decentralml/assets` in your home folder (or the home folder of the user you are going to use to run the dockers)
- If you are simulating the remote storage, also create a folder and subfolder `decentralml/remote` in the same home folder.
- If you want to test some examples, copy the content of the `asset` folder in the python client `substrate-client-decentralml` in the previously created folder `decentralml/assets`

## Run node and client

To run the node and client, use the provided `docker-compose.yml` file provided in this repo. To launch the node and all the clients, one for each role, from a terminal pointed at the same location of the `docker-compose.yml` file, execute the command:

```docker compose up```

> **_NOTE:_**  Make sure you have a folder called `assets` within a folder called `DecentralML` in your home folder, as it is used by the client to upload files for the tasks.

Now you can open a **second terminal** and attach a shell to the client container using the command:

```docker attach model_creator```

You can attach a shell for any of the role, with **one** of the following commands:
```
docker attach model_contributor
docker attach model_engineer
docker attach data_annotator
```

From the attached shell, you can then run the decentralml client with:

```bash
python -m decentralml.main
```

The menu will present you the options for the corresponding role.

## File description

The `docker` folder contains the following file:

```
.
├── README.md
├── build_client.sh
├── build_node.sh
├── client_container
│   ├── Dockerfile
│   └── launch_client.sh
├── decentralml_app
│   ├── compose_data_annotator.yml
│   ├── compose_model_contributor.yml
│   ├── compose_model_creator.yml
│   ├── compose_model_engineer.yml
│   ├── compose_node_decentralml.yml
│   └── docker-compose.yml
└── node_container
    ├── Dockerfile
    └── launch_node.sh
```

In which:

- `build_node.sh` and `build_client.sh` are Unix scripts to build the docker image for the node and client respectively.
- `client_container` is a folder containing the description of the client image in `Dockerfile` and the command to be launch in the container created from this image `launch_client.sh`
- `node_container` is a folder containing the description of the node image in `Dockerfile` and the command to be launch in the container created from this image `launch_client.sh`
- `docker-compose.yml` in `decentralml_app` is a file that describes how the microservice infrastructure is built at runtime. Running the entire application with the method described at the beginning of this documentation, is the advised method.

    In the docker-compose.yml file, five services are defined, one for the node and one for each role (i.e. `model_creator`, `model_engineer`, `model_contributor`, `data_annotator`). All the services shares the definition of the following parameters:
        
    - `image`: this define what image must be used to create the container for the service
    - `container_name`: the name of the container that is created
    - `hostname`: the hostname of the system inside the container. It helps identify the service from the terminal once connected.
    - `links`: this assures that all the services are run under the same network.
    - `stdin_true` and `tty`: this assures that a container keeps running with a attachable shell once launched. This makes possible for a terminal to be connected to a running container.

    In addition, the node service defines:

    - `ports`: this parameters link the ports in the container with post on the host system. In this way, it is possible to use the node running in the container even when the clients are running locally on the host.

    For each of the roles containers, the following parameters are also defined:

    - `volumes`: this defines the link between the folders inside the container, with folders on the host system.
    - `environment`: in this section, all the environmental variable are defined. This is the best way to set parameters needed by the services at runtime. These variables can then be read by the python code (i.e. look at the file `substrate-client-decentralml/src/decentralml/settings.py` to see how the default variables are read.)

    Finally, single docker compose files are provided to run each component singularly. If you only want to run a component, you can use the command:
    ```bash
    docker compose -f xxxxx up
    ```
    where `xxxxx` is the name of the file for that component (i.e. `compose_node_decentralml.yml`, `compose_model_creator.yml`).





