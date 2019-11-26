# ink! - Parity's ink to write smart contracts

|       Linux        |       Codecov        |       Coveralls        |       LoC        |
| :----------------: | :------------------: | :--------------------: | :--------------: |
| [![linux][a1]][a2] | [![codecov][c1]][c2] | [![coveralls][d1]][d2] | [![loc][e1]][e2] |

[a1]: https://gitlab.parity.io/parity/ink/badges/master/build.svg
[a2]: https://gitlab.parity.io/parity/ink/pipelines
[c1]: https://codecov.io/gh/paritytech/ink/branch/master/graph/badge.svg
[c2]: https://codecov.io/gh/paritytech/ink/branch/master
[d1]: https://coveralls.io/repos/github/paritytech/ink/badge.svg?branch=master
[d2]: https://coveralls.io/github/paritytech/ink?branch=master
[e1]: https://tokei.rs/b1/github/paritytech/ink?category=code
[e2]: https://github.com/Aaronepower/tokei#badges
[f1]: https://img.shields.io/badge/docs-core-blue.svg
[f2]: https://paritytech.github.io/ink/ink_core
[g1]: https://img.shields.io/badge/docs-model-blue.svg
[g2]: https://paritytech.github.io/ink/ink_model
[h1]: https://img.shields.io/badge/docs-abi-blue.svg
[h2]: https://paritytech.github.io/ink/ink_abi

**IMPORTANT NOTE:** WORK IN PROGRESS! Do not expect this to be working.

ink! is an [eDSL](https://wiki.haskell.org/Embedded_domain_specific_language) to write WebAssembly based smart contracts using the Rust programming language targeting Substrate blockchains.

For more information please visit [the ink! tutorial](https://substrate.dev/substrate-contracts-workshop/#/0/building-your-contract).

## Developer Documentation

| `ink_abi`     | `ink_core`    | `ink_model`   |
| ------------- | ------------- | ------------- |
| [![][h1]][h2] | [![][f1]][f2] | [![][g1]][g2] |

### Interaction with Substrate

Susbtrate contains the `srml-contracts` module, which provides a generic
smart contract interface for WASM blobs. It takes care of e.g. state rent,
storage, costs, etc..

ink! is a smart contract language which targets the interface exposed by
`srml-contracts`. As such, ink! smart contracts are compiled to WASM.

### Scripts

Use the scripts provided under `scripts` directory in order to run checks on either the workspace or all examples. Please do this before pushing work in a PR.

## Examples

For building the example smart contracts found under `examples` you will need to have [`cargo-contract`](https://github.com/paritytech/cargo-contract) installed.

```
cargo install --git https://github.com/paritytech/cargo-contract cargo-contract --force
```

Use the `--force` to ensure you are updated to the most recent `cargo-contract` version.

### Build example contract and generate the contracts metadata

To build a single example and generate the contracts Wasm file, navigate to the root of the example smart contract and run:

```
cargo contract build
```

To generate the contract metadata (a.k.a. the contract ABI), run the following command:

```
cargo contract generate-metadata
```

You should now have an optimized `<contract-name>.wasm` file and an `metadata.json` file in the `target` folder of the contract.

For further information, please have a look at our [smart contracts workshop](https://substrate.dev/substrate-contracts-workshop/).

## Hello, World! - The Flipper

The `Flipper` contract is a simple contract containing only a single `bool` value
that it can flip from `true` to `false` and vice versa and return the current state.

To create your own version of the flipper contract, you first need to initialize a new ink! project in your working directory.

```
cargo contract new flipper
```

Below you can see the code using the `ink_lang2` version of ink!.

```rust
use ink_core::storage;
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod flipper {
    /// The storage of the flipper contract.
    #[ink(storage)]
    struct Flipper {
        /// The single `bool` value.
        value: storage::Value<bool>,
    }

    impl Flipper {
        /// Instantiates a new Flipper contract and initializes `value` to `init_value`.
        #[ink(constructor)]
        fn new(&mut self, init_value: bool) {
            self.value.set(init_value);
        }

        /// Instantiates a new Flipper contract and initializes `value` to `false` by default.
        #[ink(constructor)]
        fn default(&mut self) {
            self.new(false)
        }

        /// Flips `value` from `true` to `false` or vice versa.
        #[ink(message)]
        fn flip(&mut self) {
            *self.value = !self.get();
        }

        /// Returns the current state of `value`.
        #[ink(message)]
        fn get(&self) -> bool {
            *self.value
        }
    }

    /// As in normal Rust code we are able to define tests like below.
    ///
    /// Simply execute `cargo test` in order to test your contract.
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn default_works() {
            // Note that `#[ink(constructor)]` functions that above have been
            // defined as `&mut self` can be used as normal Rust constructors
            // in test mode.
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        #[test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }
    }
}
```

Place this code in the `./lib.rs` file of your flipper contract and run `cargo contract build && cargo contract generate-metadata` to build your first ink! smart contract example.

## Contribution

Visit our [contribution guidelines](CONTRIBUTING.md) for more information.

## License

The entire code within this repository is licensed under the [Apache License 2.0](LICENSE). Please [contact us](https://www.parity.io/contact/) if you have questions about the licensing of our products.
