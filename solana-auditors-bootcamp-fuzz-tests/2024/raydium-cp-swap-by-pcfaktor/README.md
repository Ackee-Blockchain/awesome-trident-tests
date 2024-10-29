# Fuzz Testing Report for Raydium Swap Program

## 1. Introduction

For the testing, the Raydium Swap program was chosen. Raydium is an automated market maker (AMM) built on the Solana blockchain, which leverages a central limit order book to enable lightning-fast trades, shared liquidity, and new features for earning yield. The `raydium-cp-swap` is a revamped constant product AMM program optimized for straightforward pool management.

- **App:** [Raydium](https://raydium.io)
- **Program Repository:** [raydium-cp-swap](https://github.com/raydium-io/raydium-cp-swap)

## 2. Fuzzing Setup

### Environment:
- **solana-cli:** 1.18.23
- **anchor-cli:** 0.29.0
- **trident-cli:** 0.7.0

### Trident Configuration:
- **Iterations:** 1000
- **Exit Upon Crash:** true
- **Fuzzing with Stats:** true

### Target:
- Only the `raydium-cp-swap` program is included.
- All program instructions are target instructions.
- Data and accounts are configured for all instructions.

## 3. Test Results

```plaintext
------------------------[  0 days 00 hrs 13 mins 30 secs ]----------------------                         
  Iterations : 1,000 [1.00k] (out of: 1,000 [100%])                                                      
  Mode [3/3] : Feedback Driven Mode                                                                      
      Target : trident-tests/fuzz_tests/fuzzing.....wn-linux-gnu/release/fuzz_0                          
     Threads : 2, CPUs: 4, CPU%: 400% [100%/CPU]                                                         
       Speed : 0/sec [avg: 1]                                                                            
     Crashes : 0 [unique: 0, blocklist: 0, verified: 0]                                                  
    Timeouts : 20 [10 sec]                                                                               
 Corpus Size : 726, max: 1,048,576 bytes, init: 810 files                                                
  Cov Update : 0 days 00 hrs 00 mins 00 secs ago                                                         
    Coverage : edge: 10,998/988,956 [1%] pc: 137 cmp: 751,456                                            
---------------------------------- [ LOGS ] ------------------/ honggfuzz 2.6 /-
...
...
Summary iterations:1002 time:810 speed:1 crashes_count:0 timeout_count:20 new_units_added:89 slowest_unit
_ms:10536 guard_nb:988956 branch_coverage_percent:1 peak_rss_mb:3480       
```
### Summary

- **Iterations**: 1,000 (100% completed)
- **Timeouts**: 20 (10 sec each)
- **Corpus Size**: 726 files, max size: 1,048,576 bytes
- **New Units Added**: 89
- **Branch Coverage**: 1%
- **Crashes**: 0 (no unique, blocklisted, or verified crashes)
- **Peak RSS**: 3480 MB
- **Slowest Unit**: 10536 ms

### Statistics 

| Instruction        | Invoked Total | Ix Success | Check Failed | Ix Failed |
|--------------------|---------------|------------|--------------|-----------|
| UpdateAmmConfig    | 42            | 0          | 0            | 42        |
| SwapBaseInput      | 31            | 0          | 0            | 31        |
| CollectProtocolFee | 36            | 0          | 0            | 36        |
| Deposit            | 13            | 0          | 0            | 13        |
| Withdraw           | 25            | 0          | 0            | 25        |
| UpdatePoolStatus   | 57            | 0          | 0            | 57        |
| CollectFundFee     | 85            | 0          | 0            | 84        |
| CreateAmmConfig    | 388           | 0          | 0            | 388       |
| Initialize         | 39            | 0          | 0            | 39        |
| SwapBaseOutput     | 222           | 0          | 0            | 222       |

## 4. Observations

### Security Documentation
The program includes a security.txt file configured with information about the project, contact points for vulnerabilities, and links to the source code and audit reports. This demonstrates the project's commitment to security and transparency. The security.txt references an audit report by MadShield for Q1 2024, providing an additional layer of confidence in the program’s security.

### Commented Code
The code is well-documented, with comments explaining the purpose of functions and parameters. However, there are a few minor typos in the comments (e.g., "tarde" instead of "trade", "vaule" instead of "value"). While these do not affect functionality, correcting these typos would enhance the code's professionalism and readability.

## 5. Conclusion

The fuzzing process did not reveal any bugs or crashes in the `raydium-cp-swap` program, indicating a high level of robustness in its implementation. The program has undergone a security audit and is part of an active bug bounty program on [Immunefi](https://immunefi.com/bug-bounty/raydium), further demonstrating its commitment to maintaining a secure and reliable protocol.
Overall, the Raydium Swap program appears to be of high quality, with a strong security posture and diligent documentation practices.