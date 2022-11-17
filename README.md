# Tic Tac Toe
Implemenattion of a Tic-Tac-Toe Game on Solana

You can play the game [here](https://shpota.com/game/). Make sure you have Phantom
installed and you are using the Devnet.

## How to use

#### Build:

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

#### Deploy:

```shell
 anchor deploy --provider.cluster devnet
```
