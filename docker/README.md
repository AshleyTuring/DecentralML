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

To run the node and client, use the provided `docker-compose.yml` file provided in this repo. To launch the node and client run, from a terminal pointed at the same location of the `docker-compose.yml` file:

```docker compose up```

Now you can open a second terminal and attach a shell to the client container using the command:

```docker attach client_decentralml```

From this shell, you can use the python scripts in the folder `/decentralml/substrate-client-decentralml` to interact with the chain.




