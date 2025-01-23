### TDX platform test

Run on linux confidential VM with TDX enabled vs. MacOS
```
cargo build
sudo ./target/debug/tdx-demo
 ```

Then take the resulting raw bytes of the TDX attestation report (that's printed to stdout)
and verify it on-chain with `verifyAndAttestOnChain(0, 0x04....<attestation_bytes>...)`:
https://sepolia.etherscan.io/address/0xE28ea4E574871CA6A4331d6692bd3DD602Fb4f76#writeContract


See more tests in the test folder:
https://github.com/automata-network/automata-dcap-attestation/blob/main/forge-test/AutomataDcapAttestationTest.t.sol