# WAVS Monorepo Template

<!-- ![Rust](https://github.com/gakonst/foundry-rust-template/workflows/Rust/badge.svg)
![Solidity](https://github.com/gakonst/foundry-rust-template/workflows/Solidity/badge.svg)
[![Telegram Chat][tg-badge]][tg-url]

[tg-badge]:
  https://img.shields.io/endpoint?color=neon&style=flat-square&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Ffoundry_rs
[tg-url]: https://t.me/foundry_rs -->

**Template for quickly getting started with developing WAVS Rust applications**

Continuous Integration is already set up to test both your Rust and Solidity
code, as well as ensure formatting and that your Rust bindings match the
Solidity build artifacts.

<!-- ## Directory Structure

The project is structured as a mixed Rust workspace with a Foundry project under
`contracts/` and typesafe auto-generated bindings to the contracts under
`crates/bindings/`.

```
├── Cargo.toml
├── app // <-- Your Rust application logic
├── crates
    └── bindings // <-- Generated bindings to the smart contracts' abis (like Typechain)
``` -->

## Testing

Given the repository contains both Solidity and Rust code, there's 2 different
workflows.

### Solidity

Forge is using submodules to manage dependencies. Initialize the dependencies:

If you are in the root directory of the project, run:

```bash
forge install
```

Then, run the tests:

If you are in the root directory of the project, run:

```bash
forge test
```

### Rust

```bash
cargo test
```

## Generating Rust bindings to the contracts

Rust bindings to the contracts can be generated via `forge bind`, which requires
first building your contracts:

```bash
forge build
make bindings
```

<!-- Any follow-on calls to `forge bind` will check that the generated bindings match
the ones under the build files. If you want to re-generate your bindings, pass
the `--overwrite` flag to your `forge bind` command. -->

## WAVS Operations

```bash
forge build
```

Run the following in the WAVS repo (seperate terminal)
```bash
# this is based on ebf8cf6bc001d8292696ef6883d55d749c41bdd9 in the WAVS repo
docker compose up # <- in the WAVS repository
```

Upload ServiceManager from this repo

```bash
export CLI_EIGEN_CORE_DELEGATION_MANAGER=`docker exec -it wavs bash -c 'jq -r .eigen_core.local.delegation_manager ~/wavs/cli/deployments.json' | tr -d '\r'`
export CLI_EIGEN_CORE_REWARDS_COORDINATOR=`docker exec -it wavs bash -c 'jq -r .eigen_core.local.rewards_coordinator ~/wavs/cli/deployments.json' | tr -d '\r'`
export CLI_EIGEN_CORE_AVS_DIRECTORY=`docker exec -it wavs bash -c 'jq -r .eigen_core.local.avs_directory ~/wavs/cli/deployments.json' | tr -d '\r'`
export FOUNDRY_ANVIL_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

forge script ./script/WavsServiceManager.s.sol --rpc-url http://localhost:8545 --broadcast
```

Build WAVS WASI component

```bash
(cd components/eth-trigger-echo; cargo component build --release)

mkdir -p ./compiled && cp ./target/wasm32-wasip1/release/*.wasm ./compiled/
```

Verify

```bash
# install wavs-cli
# https://github.com/Lay3rLabs/WAVS/issues/277
# - right now `E0308` is super annoying @ packages/wavs/src/submission/core.rs:426:13 ^ have to do a `result: result.into(),` for now.
(cd lib/WAVS; cargo install --path ./packages/cli)

cp ./lib/WAVS/packages/cli/wavs-cli.toml .

# Local Exec
# ./compiled/eth_trigger_echo.wasm does not work because it's relative in `read_component(path: impl AsRef<Path>) -> Vec<u8> ...  Path::new("../../")`
wavs-cli exec --component $(pwd)/compiled/eth_trigger_echo.wasm --input testing


# Actual deploy
# Service manager is from the script/WavsServiceManager.s.sol broadcast logs
export WAVS_CLI_ETH_MNEMONIC="test test test test test test test test test test test junk"
wavs-cli deploy-service --component $(pwd)/compiled/eth_trigger_echo.wasm  --service-manager 0x851356ae760d987E095750cCeb3bC6014560891C --data ./.docker

wavs-cli add-task --service-id 019476fe0fe27d60af2a2af6212ca38a --input "hello testing!" --data ./.docker
```


## Installing Foundry

First run the command below to get `foundryup`, the Foundry toolchain installer:

```sh
curl -L https://foundry.paradigm.xyz | bash
```

Then, in a new terminal session or after reloading your `PATH`, run it to get
the latest `forge` and `cast` binaries:

```sh
foundryup
```

For more, see the official
[docs](https://github.com/gakonst/foundry#installation).
