<p align="left">
  <img height="100" width="100" src="https://abchprod.wpengine.com/wp-content/uploads/2024/05/Trident-Color.png" alt="Trident"/>
</p>

# Trident Tests for Raydium CP Swap
This repository serves as an example of tests written with the Solana Fuzzing Framework [Trident](https://github.com/Ackee-Blockchain/trident).


## Setup

1. Clone this repository
2. Install Trident and required dependencies with the help of this [guide](https://ackee.xyz/trident/docs/0.7.0/getting-started/getting-started/)
3. Use `--version` to specify Trident version during the installation `cargo install trident-cli --version 0.X.0`
4. Use `trident fuzz run <TARGET_NAME>` (e.g., fuzz_0) to run the fuzz test

Tested with `trident` version `0.7.0`. Fuzz tests are located inside [fuzz_tests](./trident-tests/fuzz_tests/) folder.
