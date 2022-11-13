# Tic Tac Toe
Tic Tac Toe Solana Program

## Deploy
Build:
```sh
solana config set --url devnet
solana config get
anchor build
```
Get the new program id:
```sh
solana address -k target/deploy/tic_tac_toe-keypair.json
```

Update Anchor.toml and lib.rs with new program id.

Build again:
```sh
anchor build
```
Deploy:
```shell
 anchor deploy --provider.cluster devnet
```
