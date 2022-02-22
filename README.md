# Foundry Rust Monorepo Template

![Rust](https://github.com/gakonst/foundry-rust-template/workflows/Rust/badge.svg)
![Solidity](https://github.com/gakonst/foundry-rust-template/workflows/Solidity/badge.svg)
[![Telegram Chat][tg-badge]][tg-url]

[tg-badge]:
  https://img.shields.io/endpoint?color=neon&style=flat-square&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Ffoundry_rs
[tg-url]: https://t.me/foundry_rs

**Template for quickly getting started with developing Rust applications that
leverage Foundry for EVM smart contract development.**

Continuous Integration is already set up to test both your Rust and Solidity
code, as well as ensure formatting and that your Rust bindings match the
Solidity build artifacts.

## Directory Structure

The project is structured as a mixed Rust workspace with a Foundry project under
`contracts/` and typesafe auto-generated bindings to the contracts under
`bindings/`.

```
├── Cargo.toml
├── app // <-- Your Rust application logic
├── contracts // <- The smart contracts + tests using Foundry
├── bindings // <-- Generated bindings to the smart contracts' abis (like Typechain)
```

## Testing

Given the repository contains both Solidity and Rust code, there's 2 different
workflows.

### Solidity

Forge is using submodules to manage dependencies. Initialize the dependencies:

```bash
git submodule update --init
```

If you are in the root directory of the project, run:

```bash
forge test --root ./contracts
```

If you are in in `contracts/`:

```bash
forge test
```

### Rust

```
cargo test
```

## Generating Rust bindings to the contracts

Rust bindings to the contracts can be generated via `forge bind`, which requires
first building your contracts:

```
forge build --root ./contracts
forge bind --bindings-path ./bindings --root ./contracts --crate-name bindings
```

Any follow-on calls to `forge bind` will check that the generated bindings match
the ones under the build files. If you want to re-generate your bindings, pass
the `--overwrite` flag to your `forge bind` command.

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
