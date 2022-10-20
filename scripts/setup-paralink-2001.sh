#!/bin/bash

# Location independent running
scriptDir=$(dirname -- "$(readlink -f -- "${BASH_SOURCE[0]}")")
RELAY_CHAIN_SPEC="$scriptDir"/../assets/polkadot-relay-chain-spec.json
PLAIN_SPEC="$scriptDir"/../assets/paralink-plain.json
RAW_SPEC="$scriptDir"/../assets/paralink-2001-raw.json
WASM_VALIDATION="$scriptDir"/../assets/paralink-2001-wasm
GENESIS="$scriptDir"/../assets/paralink-2001-genesis

# Build chain
(cd "$scriptDir"; cargo build --release)

# Generate plain spec
"$scriptDir"/../target/release/paralink-node build-spec --disable-default-bootnode > "$PLAIN_SPEC"

# Fix the parachain ID to 2001
sed -i 's@para_id": 1000@para_id": 2001@g' "$PLAIN_SPEC" 
sed -i 's@parachainId": 1000@parachainId": 2001@g'  "$PLAIN_SPEC"

# Reserve the ParaId through https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/parachains/parathreads

# Generate raw spec from updated plain file
"$scriptDir"/../target/release/paralink-node build-spec --chain "$PLAIN_SPEC" --raw --disable-default-bootnode > "$RAW_SPEC"

# Generate validation function and genesis state
"$scriptDir"/../target/release/paralink-node export-genesis-wasm --chain "$RAW_SPEC" > "$WASM_VALIDATION"
"$scriptDir"/../target/release/paralink-node export-genesis-state --chain "$RAW_SPEC"> "$GENESIS"

# Start the chain
"$scriptDir"/../target/release/paralink-node --collator --bob --force-authoring --tmp --port 40337 --ws-port 9948 --rpc-external --ws-external --rpc-cors all --rpc-methods=Unsafe  --chain "$RAW_SPEC" -- --execution wasm --chain "$RELAY_CHAIN_SPEC" --port 30337
