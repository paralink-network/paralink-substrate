![](https://paralink.network/images/logo-sm-home.png)

## Getting Started

This repository contains Substrate based runtime for Paralink Network.

### Makefile

This project uses a [Makefile](Makefile) to document helpful commands and make it easier to execute them.

1. `make init` - Configures the correct Rust toolchain for
   [WebAssembly compilation](https://substrate.dev/docs/en/knowledgebase/getting-started/#webassembly-compilation).
2. `make build` - Build the chain in release mode.
3. `make run` - Runs the chain in release mode. However, a local relay chain has to be setup first. See [paralink-xcm](https://github.com/paralink-network/paralink-xcm) repo and run the parachain from there.
4. `make generate-specs` - generates specification for the parachain.

### Build

```sh
make build
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and subcommands:

```sh
./target/release/paralink-node -h
```

## Run

The `make run` command will launch the Paralink parachain and will try to connect to the local relay chain. See the scripts `run-paralink.sh` in `scripts`. Use [paralink-xcm](https://github.com/paralink-network/paralink-xcm) repo to setup the whole environment.
