# Fly
Rust implementation for [fly.io/dist-sys](https://fly.io/dist-sys) problems.

This is mostly to write some Rust code while learning it. So it's gonna be shabby.

## Setup
1. Follow instructions on fly.io to setup maelstrom. 
2. Compile binary
3. Run test
Eg:
```sh
cargo build
java -jar target/maelstrom-0.2.4-SNAPSHOT-standalone.jar test -w echo --bin ~/Workspaces/Pwned/Rust/fly/target/debug/fly  --node-count 1 --time-limit 10
```

## Checklist
- [ ] FSM?
- [ ] Make the message struct fields public for easier creation / access
- [ ] Multithreading support for nodes
