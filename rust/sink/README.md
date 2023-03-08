# Sink

Is a specialized library to handle data from the nasl-interpreter to the storage / distribution implementation.

To be able to introduce new distribution implementations the [Sink](src/lib.rs#L80) must be implement.

The [Dispatch](src/lib.rs#L21) enum required by `dispatch` method describes fields to be distributed.

The [Retrieve](src/lib.rs#L36) enum required by `retrieve` method also describes the fields to be retrieved.

The reason that it uses field descriptions rather than structs are two fold:
1. it allows the usage of streaming distribution
2. it makes it easier to store information immediately on execution

Since we sometimes have the requirement to just store when all information is available the [on_exit](src/lib.rs#L93) must be called when the interpreter finishes.

A simplified example on how to write a Sink implementation can be found in [DefaultSink](src/lib.rs#L116)

## Build

Run `cargo test` to test and `cargo build --release` to build it.