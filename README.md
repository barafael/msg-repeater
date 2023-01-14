# Message Repeating TCP Server

Super-simple server that keeps repeating a message with a specified period.

## Tokio variant

Build with `cargo b --features async-spawning` for the tokio-based variant.

## Running it

`cargo r [--features async-spawning] -- raw 32 33 34 a`

In another shell:

`telnet localhost 8080`

See `cargo r -- --help` for CLI args.