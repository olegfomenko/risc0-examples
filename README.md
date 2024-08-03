# RISC Zero Pedersen commitment verification example

This example shows how to verify computation of the Pedersen commitment that consumes private key and amount (also in a
couple with group elements G and H) and outputs the commitment in journal.

## Quick Start

First, make sure [rustup] is installed. The
[`rust-toolchain.toml`][rust-toolchain] file will be used by `cargo` to
automatically install the correct version.

To build all methods and execute the method within the zkVM, run the following
command:

```bash
cargo run --release
```

### Executing the project locally in development mode

During development, faster iteration upon code changes can be achieved by leveraging [dev-mode], we strongly suggest
activating it during your early development phase. Furthermore, you might want to get insights into the execution
statistics of your project, and this can be achieved by specifying the environment variable `RUST_LOG="[executor]=info"`
before running your project.

Put together, the command to run your project in development mode while getting execution statistics is:

```bash
RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run
```

### Running proofs remotely on Bonsai

_Note: The Bonsai proving service is still in early Alpha; an API key is
required for access. [Click here to request access][bonsai access]._

If you have access to the URL and API key to Bonsai you can run your proofs
remotely. To prove in Bonsai mode, invoke `cargo run` with two additional
environment variables:

```bash
BONSAI_API_KEY="YOUR_API_KEY" BONSAI_API_URL="BONSAI_URL" cargo run
```

## Directory Structure

It is possible to organize the files for these components in various ways.
However, in this starter template we use a standard directory structure for zkVM
applications, which we think is a good starting point for your applications.

```text
project_name
├── Cargo.toml
├── host
│   ├── Cargo.toml
│   └── src
│       └── main.rs                    <-- [Host code goes here]
└── methods
    ├── Cargo.toml
    ├── build.rs
    ├── guest
    │   ├── Cargo.toml
    │   └── src
    │       └── method_name.rs         <-- [Guest code goes here]
    └── src
        └── lib.rs
```

## Video Tutorial

For a walk-through of how to build with this template, check out this [excerpt
from our workshop at ZK HACK III][zkhack-iii].

## Questions, Feedback, and Collaborations

We'd love to hear from you on [Discord][discord] or [Twitter][twitter].

[bonsai access]: https://bonsai.xyz/apply

[cargo-risczero]: https://docs.rs/cargo-risczero

[crates]: https://github.com/risc0/risc0/blob/main/README.md#rust-binaries

[dev-docs]: https://dev.risczero.com

[dev-mode]: https://dev.risczero.com/api/generating-proofs/dev-mode

[discord]: https://discord.gg/risczero

[docs.rs]: https://docs.rs/releases/search?query=risc0

[examples]: https://github.com/risc0/risc0/tree/main/examples

[risc0-build]: https://docs.rs/risc0-build

[risc0-repo]: https://www.github.com/risc0/risc0

[risc0-zkvm]: https://docs.rs/risc0-zkvm

[rustup]: https://rustup.rs

[rust-toolchain]: rust-toolchain.toml

[twitter]: https://twitter.com/risczero

[zkvm-overview]: https://dev.risczero.com/zkvm

[zkhack-iii]: https://www.youtube.com/watch?v=Yg_BGqj_6lg&list=PLcPzhUaCxlCgig7ofeARMPwQ8vbuD6hC5&index=5
