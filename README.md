# arb-airdrop-check

CLI used to check address eligibility as part of the arbitrum airdrop. Expects a file containing a single address per line:

1) at addresses to file
```bash
$> "echo ......." > addresses.txt
```
2) Compile code
```bash
$> cargo build --release && cp target/release/arb-airdrop-check .
```
3) Check your self before you wrekt yourself!
```bash
$> ./arb-airdrop-check
```

