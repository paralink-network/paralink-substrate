#!/bin/bash
# Runs the parachain, assumes relay chain is already running, see paralink-xcm repo `make run_relay` command

scriptDir=$(dirname -- "$(readlink -f -- "${BASH_SOURCE[0]}")")
RELAY_CHAIN_SPEC="$scriptDir"/../assets/polkadot-relay-chain-spec.json
RAW_SPEC="$scriptDir"/../assets/paralink-2001-raw.json

# Build chain
(cd "$scriptDir"; cargo build --release) || exit $?

echo "$scriptDir"

# Start the chain
"$scriptDir"/../target/release/paralink-node --collator --bob --force-authoring --tmp --port 40337 --ws-port 9948 --rpc-external --ws-external --rpc-cors all --rpc-methods=Unsafe  --chain "$RAW_SPEC" -- --execution wasm --chain "$RELAY_CHAIN_SPEC" --port 30337
