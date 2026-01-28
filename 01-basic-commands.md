# 01-Basic-commands aka. anchor

### NodeJS

#### Installing dependencies

```sh
npm clean-install
```

### Solona CLI

#### Set initial config in Solona CLI

```sh
solana config get
```

### View transaction logs

```sh
solana logs
```

#### Changing networks

You can choose between:

- mainnet-beta
- devnet
- localhost
- testnet

```sh
solana config set --url {network}
```

#### Create wallet

The following command will show your private key for the wallet ig;
it is worth writing it down somewhere safe.

```sh
solana-keygen new
```

#### View wallet address

```sh
solana address
```

#### View balance

```sh
solana balance
```

#### View wallet private key

In case if you forgot it you can still view it with this command

```sh
solana confirm -v
```

### Only on devnet

#### Request devnet airdrop

Has rate limitation only and 5 Sol/request

```sh
solana airdrop {amount}
```

### Anchor CLI

#### Project creation

```sh
anchor init <project-name>
```

if you created to be rust only:

```sh
anchor init <project-name> rust
```

#### Building program

In project's directory

```sh
anchor build
```

If you got an error about the paths run this command:

```sh
cargo-build-sbf --force-tools-install
```

This will reinstall all the tools

#### Deploy program

In project's directory

```sh
anchor deploy
```

#### Program testing

##### Test program

In project's directory

```sh
anchor test
```

##### Test program with already runing validator

If the local validator is already running use it with `--skip-local-validator` switch

```sh
anchor test --skip-local-validator
```

### Surfpool

#### Surfpool startup

This is our local validator (on `anchor test` this will start and stop automatically)

```sh
surfpool start
```

If you want it to run in background use this command

```sh
surfpool start&
```

You will get a web ui (called Surfpool Studio) on port 18488

<http://127.0.0.1:18488>

You can set it to run on 0.0.0.0 by starting with argument `--host 0.0.0.0`

If you are (not) ready to dive in there is the link for the docs

[Go to next step (hello world)](02-hello-world.md)
