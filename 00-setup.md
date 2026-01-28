# 00-Setup

I need this because everybody uses AVX2 what is rust fu rust

```sh
sudo apt-get update
```

#### Installing base packages

```sh
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libudev-dev llvm libclang-dev \
    protobuf-compiler libssl-dev
```

#### Installing rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

```sh
. "$HOME/.cargo/env"
```

Verification command:

```sh
rustc --version
```

#### Installing solonaCLI

Check it if you have the following cpu capabilities:

- AVX2 
- AVX 
- SSE
- SSE2

by running:

```sh
cat /proc/cpuinfo | grep flags | grep {capability}
```

If its empty any then you dont have it

<details>
  <summary>If you have it</summary>
Lucky basterd

Install Solona CLI

```sh
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
```

Then close your terminal and reopen it

</details>

<details>
  <summary>If you dont have it</summary>
Installing compile dependencies

```sh
sudo apt-get install -y \
    libssl-dev libudev-dev pkg-config zlib1g-dev llvm \
    clang cmake make libprotobuf-dev protobuf-compiler git
```

Cloning the src repo:

```sh
git clone https://github.com/anza-xyz/agave.git --depth=1
```

Compiling it (grab a cup of someting that will take some time)

If you go for speed:

```sh
export RUST_REED_SOLOMON_ERASURE_ARCH=native
export RUSTFLAGS="-C target-cpu=native -C target-feature=-avx,-avx2"
cd agave && ./scripts/cargo-install-all.sh .
```

If you go for compatiblity:

```sh
export RUST_REED_SOLOMON_ERASURE_ARCH=x86-64
export RUSTFLAGS="-C target-feature=-avx,-avx2"
cd agave && ./scripts/cargo-install-all.sh .
```

the command which sets the `RUSTFLAGS` you can modify it
to disable certain capability requirements by appending it to it:
`target-feature=-...,-{capability}`

#### Testing it if it works

```sh
./bin/solana-test-validator
```

if it errors with `Illegal instruction (core dumped)` you dont have luck,
check your disabled capabilities flag and modify it with by;
fully execute again the export line:

```sh
export RUSTFLAGS="-C target-feature=-avx,-avx2"
```

Then to restart build first cleanup, then build:

```sh
cargo clean
./scripts/cargo-install-all.sh .
```

Moving binaries to path

```sh
cp -r bin/* $HOME/.local/share/solana/install/active_release/bin
```

</details>

##### Adding solonaCLI to path

```sh
SOLANA_PATH="$HOME/.local/share/solana/install/active_release/bin"
export PATH="$SOLANA_PATH:$PATH"
echo "export PATH=\"$SOLANA_PATH:\$PATH\"" >> ~/.bashrc
source ~/.bashrc
```

Verification:

```sh
solana --version
```

Update command (optional):

```sh
agave-install update
```

#### Installing Yarn via NodeJS with NVM (Optional) but recommended for testing

If you skip this use only `anchor init {project_name} rust`

##### Installing NVM

```sh
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh | bash
```

NVM Verification

```sh
command -v nvm
```

##### Installing NodeJS

```sh
nvm install node
```

Verification command:

```sh
node --version
```

##### Installing Yarn via NodeJS

```sh
npm install --global yarn
```

Verification command:

```sh
yarn --version
```

#### Installing Anchor CLI via Anchor version manager

##### Installing Anchor version manager

```sh
cargo install --git https://github.com/solana-foundation/anchor avm --force
```

Verification command:

```sh
avm --version
```

##### Installing Anchor CLI

```sh
avm install latest
```

This will set automatically set it to use the latest version if not

```sh
avm use latest
```

Verification command:

```sh
anchor --version
```

#### Installing Surfpool CLI

Idk what for but the original tutorial has it

```sh
curl -sL https://run.surfpool.run/ | bash
```

Verification command

```sh
surfpool --version
```

#### For sanity (Optional but STRONGLY recommended)

##### Caching the built packages

The package will cache the already built required packages for the projects

This package will do the job

```sh
cargo install sccache --locked
```

Then make it rust use sccache package

```sh
echo 'export RUSTC_WRAPPER=sccache' >> ~/.bashrc
source ~/.bashrc
```

You can also limit it how much space use it for cache with this command

```sh
export SCCACHE_CACHE_SIZE="20G"
```

for save it to global config:

```sh
echo 'export SCCACHE_CACHE_SIZE="20G"' >> ~/.bashrc
source ~/.bashrc
```

You can watch it work with this command

```sh
sccache --show-stats
```

This will display how much cache hit or miss happened

[Go to next step (Basic commands)](01-basic-commands.md)
