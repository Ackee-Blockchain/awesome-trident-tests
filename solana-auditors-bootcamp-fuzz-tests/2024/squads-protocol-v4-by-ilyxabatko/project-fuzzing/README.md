# Fuzz Testing for Solana Squads v4 Protocol

## Overview

This repository contains fuzz tests for the **Squads v4** protocol, an upgraded version of the multisig protocol on Solana. Squads v4 introduces new features such as time locks, spending limits, roles, sub-accounts, multi-party payments, and address lookup table support.

Fuzz testing was implemented using the **Trident Fuzz Testing Framework** to evaluate the robustness of the protocol by testing various critical functions with randomly generated inputs. The goal was to uncover potential vulnerabilities or edge cases that might be overlooked by standard testing practices.

## Fuzz Testing Setup

### Selected Protocol
- **Protocol**: Solana Squads v4 Protocol

### Fuzzing Tool
- **Tool**: Trident Fuzz Testing Framework

### Target Functions
Key functions related to multisig management, spending limits, and time lock logic were selected for fuzz testing. These functions are central to the operation of the protocol and critical to its security.

### Installation & Configuration

1. Clone the repository and navigate to the project directory:
   ```bash
   git clone https://github.com/Solana-Auditors-Bootcamp/fuzzing-with-trident-ilyxabatko
   cd fuzzing-with-trident-ilyxabatko/project-fuzzing/squads-v4
   ```

2. Set up Trident by following its documentation, ensuring it integrates with the Anchor-based Squads v4 project:
[Trident Docs: Getting Started](https://ackee.xyz/trident/docs/latest/getting-started/getting-started/)

3. Run the fuzz tests:
    ```bash
    trident fuzz run fuzz_3
    ```

## Fuzz Testing Results
- Number of iterations: 5,000
- Time spent: ~15 minutes
- Crashes or Panics: None detected
- Issues found: No vulnerabilities or edge cases were uncovered during the fuzzing process.

## Conclusion
The fuzz testing did not reveal any bugs or vulnerabilities in the Squads v4 protocol after 5,000 iterations over a 15-minute testing window. The protocol appears to handle random and unexpected inputs gracefully. However, further fuzzing over longer periods and deeper investigation into additional functions may uncover more subtle issues. Some of the edge case might've been skipped because of the lack Triden deep knowledge and auditing skills. 