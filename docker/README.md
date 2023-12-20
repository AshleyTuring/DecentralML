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

    Additional docker compose files are provided to run each component singularly. If you only want to run a component, you can use the command:
    ```bash
    docker compose -f xxxxx up
    ```
    where `xxxxx` is the name of the file for that component (i.e. `compose_node_decentralml.yml`, `compose_model_creator.yml`).





