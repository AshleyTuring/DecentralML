# DecentralML

A Polkadot protocol for decentralised federated machine learning and collective safety consensus

### Overview

One-liner: A Polkadot protocol for decentralised federated machine learning and collective safety consensus.

Project Description:
DecentralML introduces decentralised federated machine learning (DFML), governed by collective consensus to the Polkadot ecosystem. Our goal is to provide a robust framework for AI model developers, organisations, and applications, enabling decentralised ownership of models while ensuring privacy and scalability. With node or on-device training protecting privacy, the ability to share data training between organisations (nodes), collaborative AI training, and "collective" governance controls, DecentralML may transform the field of machine learning for state-of-the-art AI models (think LLMs and more) with transparent governance.

Integration with Substrate / Polkadot / Kusama:
DecentralML is built upon the Substrate framework, and leverages Tensorflow's extenisve Federated Machine Learning library enabling seamless integration into the Polkadot ecosystem. By leveraging Substrate's flexible and modular pallet architecture, we can shortcut a lot of the overhead needed to create a chain that has a dynamic "collective" consensus governance approach for things like AI model weights and other safety measures along with higher level controls for licensing of the entire models, jurisdiction training rules and other multi-territorial controls.

The project aims to be as open and flexible as possible to integrate with other project with an innovative "bring your own" (BYO) token staking economy (faciliated by Pallets such as Balances, Grandpa, Ink! etc).

We hope for active engagement from the wider Polkadot ecosystem and developer network once the project is complete or even during its development. We strongly believe that this project holds the transformative power to revolutionise the entire AI industry!

Team Motivation:
Our team is driven by the urgent need to challenge the dominance of centralised corporates like Facebook, OpenAI, Microsoft etc in the field of AI model development. These companies have built powerful models that require significant compute power and data, are not environmentally efficient, limit access to AI capabilities and, potentially, create a safety threat to humankind. We believe that these models could, not only be statistically improved if they were decentralised, but also may improve power usage efficiency and reduce saftey concerns by being transparently controlled by humankind on-chain, rather than a select number of corporations.

Our second motivation as Livetree, involves a solution for our AI tasks such as video processing: speech-to-text, facial recognition, scene detection and content recommendations. We currently solve these challenges using centralised model solutions and would like to make them more transparent. For further demonstrations or information on Livetree, feel free to download the free Livetree app and try speech-to-text or contact us. We will gladly provide instructions on how you can try the AI models within our our app and provide the raw AI JSON processing results for object detection, landmark recognition, speech-to-text and other AI processor outputs.

We are passionate about decentralisation and see the limitations of centralisation in terms of quality of the models, data ownership, privacy, and safety control. This has fueled our motivation to decentralise these models and create a decentralised federated machine learning substrate:

# DecentralML pallet

This is the DecentralML pallet implementation.

## Getting Started

### Run

First, complete the [basic Rust setup instructions](https://docs.substrate.io/install/).
Switch into the "substrate-node-decentralml" folder if you are not already
Use Rust's native `cargo` command to build and launch the DecentralML pallet:

```sh
 cargo build -p pallet-decentralml
```

### Build

The `cargo build` command will perform a build on just the pallet-decentralml. Use the following command to build the node
without launching it:

```sh
cargo build --package node-decentralml --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/decentralml -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with non-persistent state:

```bash
 ./target/release/node-decentralml --dev
```

Purge the development chain's state:

```bash
./target/release/node-decentralml purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/decentralml -ldebug --dev
```

> Development chain means that the state of our chain will be in a tmp folder while the nodes are
> running. Also, **alice** account will be authority and sudo account as declared in the
> [genesis state](https://github.com/livetreetech/DecentralML/blob/main/substrate-node-decentralml/runtime/src/lib.rs).
> At the same time the following accounts will be pre-funded:
>
> - Alice
> - Bob
> - Alice//stash
> - Bob//stash

In case of being interested in maintaining the chain' state between runs a base path must be added
so the db can be stored in the provided folder instead of a temporal one. We could use this folder
to store different chain databases, as a different folder will be created per different chain that
is ran. The following commands shows how to use a newly created folder as our db base path.

```bash
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/node-decentralml --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```

### Connect with Polkadot-JS Apps Front-end

Once the DecentralML node is running locally, you can connect it with **Polkadot-JS Apps** front-end to
interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local DecentralML node.

### Unit Test

Run `cargo test` command to build and launch all unit tests. The unit tests will stop if all tests
pass or any of them fails.

```sh
cargo test
```

## Python Client

There are several python scripts that correlate to create_task, assign_task, send_task_result, validate_task_result, accept_task_result, reject_task_result, list_tasks, list_task_results which allows interaction with Substrate DecentralML node and custom pallets.

It exposes and provides methods to communicate to fund actors involved and execute several tasks on the node.

This is achieved by installing the substrate-interface package into your distribution.

### Prerequisites

Python, pip and substrate-interface package are required to run the script.

After cloning the repository, switch to the decentraml/substrate-client-decentralml directory.

```bash
cd {checkout_directory}DecentralML/substrate-client-decentralml
```

- Install python [Installation] https://www.datacamp.com/blog/how-to-install-python

- Install pip [Installation] https://pip.pypa.io/en/stable/installation/

- Install substrate-interface package by running pip command:

```bash
pip install substrate-interface
```

- To execute the client script, run the following command:

```bash
python create_task.py
```

Please refer to the [python substrate interface documentation](https://pypi.org/project/substrate-interface/1.4.1/)
for API reference documentation

## Storage of assets

Infura (IPFS storage type) is used to upload and retrieve assets that tasks will use to execute their work from the Python client.

These assets pertain to files and data the different actors need to fulfill their role.

Please refer to the [Infura Documentation](https://docs.infura.io/)
for API reference documentation

## Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, followed by the whole unit test run, and then start a
local development network.

## License

[Apache 2.0 License](https://github.com/livetreetech/DecentralML/blob/main/LICENSE)
