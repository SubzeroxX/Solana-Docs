# 01a-Basic-commands-native

### Creating program

```sh
cargo new hello_world --lib
```

#### Installing dependencies

```sh
cargo add solana-program@2.2.0
```

#### Settings create type

Add these lines to Cargo.toml

```toml
[lib]
crate-type = ["cdylib", "lib"]
```

#### Build the program

```sh
cargo build-sbf
```

#### Show the program id

```sh
solana address -k ./target/deploy/{project-name}-keypair.json
```

### Testing

#### Installing testing dependecies

```sh
cargo add litesvm@0.6.1 --dev
cargo add solana-sdk@2.2.0 --dev
```

#### Testing the program

```sh
cargo test -- --no-capture
```

### Deploying

#### Deploying the program

```sh
solana program deploy ./target/deploy/{project-name}.so
```

This will display the Program Id

#### Confirm the deployment

```sh
solana program show {program-id}
```
