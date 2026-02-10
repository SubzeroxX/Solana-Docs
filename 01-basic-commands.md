# 01-Basic-commands aka. anchor

### NodeJS

#### Installing dependencies

```sh
npm clean-install
```

### Solona CLI

#### Using an another wallet for command

We can do it by pointing to a new wallet's keypair by using `--keypair keypair.json` argument

#### Importing an existing

##### Via private key

We can use a python program like the following:

<details>
<summary>Python code:</summary>

```py
# Save this as decode.py and run: python decode.py "YOUR_BASE58_PRIVATE_KEY_HERE"
import base58
import json
import sys

if len(sys.argv) < 2:
    print("Usage: python decode.py 'PRIVATE_KEY_HERE'")
    sys.exit(1)

privkey_b58 = sys.argv[1]
decoded = base58.b58decode(privkey_b58)
# Phantom private keys are 64 bytes (32 seed + 32 pubkey)
keypair = list(decoded[:32] + decoded[32:])  # but usually just the full 64 for Solana import
print(json.dumps(keypair))
```

</details>

Then we save its output keypair into a .json file which will have the wallet's private and public key 

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
- mainnet "https://api.mainnet.solana.com/"

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
solana address
```

#### View the deploy fee

Firstly start the local validator [Hint](#surfpool-startup)

Compile the program [Hint](#building-program)

View the size of the deploy with:

```sh
wc -c target/deploy/{project}.so
```

This will spit out the following output 
(size of the program and the program name) in the following format:
`XXXXX target/deploy/{project}.so`

Then for the final estimated fee

```sh
solana rent {byte lenght}
```

This could be optimzed

#### Program size optimalization for deploy


#### Deploy program to network

Firstly set network [Hint](01-basic-commands.md#changing-networks)

#### View not finished program buffers

```sh
solana program show --buffers
```

This will show a table with the buffer address and the deploy account for that program

Buffer Address | Authority  | Balance
BUFFER ADDRESS | PUBLIC KEY | X.XXX SOL

#### Continue deployment after 'error'

Eg. the deploy timouts with

Silimar message like this:

```
thread 'main' (22503) panicked at /home/user/agave/cli/src/program.rs:3345:34: 
Should return a valid tpu client: Custom("Failed find any cluster node info for upcoming leaders, timeout: 20s.") 
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Get the buffer address of the failed project [Hint](01-basic-commands.md#view-not-finished-program-buffers)

With the following command you can resume its upload

```sh
solana program deploy --buffer buffer-address target/deploy/{project}.so
```

### Only on devnet or testnet

Request airdrop for your network [Hint](01-basic-commands.md#request-devnet-or-testnet-airdrop)

### Every time needed

Syncing keys for the network [Hint](#syncing-program-id-with-the-network)

To deploy program use:

```sh
solana program deploy target/deploy/{project}.so --commitment confirmed
```

#### Request devnet or testnet airdrop

Has rate limitation only and 1.5 or 5 Sol/request

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

#### Syncing program ID with the network

```sh
anchor keys sync
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
