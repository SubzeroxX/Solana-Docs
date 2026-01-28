# 02a-Hello-world-native

After that much pain you are here GJ

#### Our first project (hello world)

Create the program called hello-world [Hint](01-basic-commands.md#project-creation)

Our main focus will be the `./programs/hello-world/src/lib.rs`
with the initial program code:

```rust
use anchor_lang::prelude::*;

declare_id!("BY8LCLmCXe8chKjiP4F8Qeryz7aZ72e6vPz8C7u4HSuh");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
```

Build the program [Hint](01-basic-commands.md#building-program)
(The first compiles are will be slow due to caching)

The tests location is `./tests/{test-name}.ts`
There is a default test case in the `./tests/hello-world.ts`
with the following code:

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloWorld } from "../target/types/hello_world";

describe("hello-world", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.helloWorld as Program<HelloWorld>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

```

To run the tests [Hint](01-basic-commands.md#test-program)

If you want to see its output:

1. Start the local validator [Hint](01-basic-commands.md#surfpool-startup)
2. Start the logging watch for trasactions [Hint](01-basic-commands.md#view-transaction-logs)
3. Run the tests [Hint](01-basic-commands.md#test-program-with-already-runing-validator)

Deploy the program [Hint](01a-basic-commands-native.md#deploying-the-program)
