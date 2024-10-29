## Fuzz Testing for ZetaChain Protocol on Solana

This README provides an overview of the fuzz testing performed on the ZetaChain protocol’s smart contracts on Solana. The fuzzing focused on the core functionalities of the protocol to identify potential vulnerabilities and logical errors.

## Protocol Overview
The ZetaChain protocol is designed for cross-chain interoperability on the Solana blockchain. This protocol’s repository can be found here (https://github.com/zeta-chain/protocol-contracts-solana/), and the specific commit used for this testing is `1495103e6ca625a6985ae622bfe241ad45ece733`.

The following functions were fuzzed:
1. `Initialize`: Initializes the protocol and sets up the necessary parameters.
2. `Deposit`: Allows a user to deposit SOL into the protocol.
3. `Withdraw`: Allows a user to withdraw SOL from the protocol.

The following functions were not fuzzed:
1. `DepositSPL`: There are currently unresolved issues related to this function, which are being discussed in the Discord channel here (https://discord.com/channels/867746290678104064/1284699416556732417).
2. `WithdrawSPL`: Similar to DepositSPL, there are unresolved issues that prevent fuzzing at the moment.

To successfully fuzz the functions, it was necessary to follow a specific sequence of instructions due to the program’s initialization requirements:
1.	`Initialization`: The Initialize function must be called first to set up the state and parameters before any other operations can be performed.
2.	`Deposit/Withdraw Operations`: Once the initialization is complete, Deposit and Withdraw functions can be called with various inputs to test their robustness and security.