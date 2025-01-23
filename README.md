### TDX platform test

Run on linux confidential VM with TDX enabled vs. MacOS
```
cargo build
sudo ./target/debug/tdx-demo
 ```

Then take the resulting raw bytes of the TDX attestation report (that's printed to stdout)
and verify it on-chain with `verifyAndAttestOnChain(0, 0x04....<attestation_bytes>...)`:
https://sepolia.etherscan.io/address/0xE28ea4E574871CA6A4331d6692bd3DD602Fb4f76#writeContract

Example successfully verified TDX attestation: https://sepolia.etherscan.io/tx/0x2ca682631d5b9caec9a8a5312818f369a5666baf8708552f6620e6aa34734f1f

Choose a [cheaper chain for attestations here](https://github.com/automata-network/automata-dcap-attestation/blob/main/README.md#testnet)
On Sepolia it cost me 0.3 Sepolia ETH


See more tests in the test folder:
https://github.com/automata-network/automata-dcap-attestation/blob/main/forge-test/AutomataDcapAttestationTest.t.sol