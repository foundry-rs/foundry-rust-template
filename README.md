# Foundry Rust Monorepo Template

Template for quickly getting started with developing Rust applications that
leverage Foundry for EVM smart contract development.

Continuous Integration is already set up to test both your Rust and Solidity
code, as well as ensure formatting and that your Rust bindings match the
Solidity build artifacts.

## Directory Structure

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
