## About

Raydium cp swap program is a constant product AMM that is used to convert from token A to token B based on the formula

A * B = K where K is a constant.

  

The workflow to use the program entails an admin creating the market config to be used by the protocol, which any user can then initialize a pool using this config.

  

The major functionalities include swapping between two tokens or adding/removing liquidity.

  

  

  

## How to build

Go to `utils.rs` in `raydium-cp-swap/trident-tests/fuzz_0` folder and set the appropriate paths in `get_admin_path()` and `get_create_pool_receiver_path()`

Run `sh fuzz.sh`
