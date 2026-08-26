# rust-learning

Repository used to learn Rust through small independent exercises and mini-projects.

Each folder is a focused example designed to practice a specific Rust concept without aiming to build a full application.

## Projets

- [basic-threading](basic-threading/) - basic multithreading example using `thread::spawn`, shared state with `Arc`, and clean thread termination with `join`.
- [ls-program](ls-program/) - small `ls`-style CLI that reads a directory path from arguments or stdin and prints entries.
- [prod-cons](prod-cons/) - producer/consumer pattern using a shared queue protected by `Mutex` and synchronized with `Condvar`.
- [units-converter](units-converter/) - simple temperature converter between Celsius and Fahrenheit, split between `main.rs` and a `conversion` module.
- [text-analyzer](text-analyzer/) - reads a text file and computes the number of lines, total words, longest word, and the 3 most frequent words.
- [traffic-light](traffic-light/) - traffic light state machine using enums, pattern matching, and manual state transitions.
- [formes](formes/) - geometry example demonstrating trait-based polymorphism with circles, rectangles, and squares.
- [config-parser](config-parser/) - parses a configuration file into structured values with custom error handling.

## Usage

Each project is independent and contains its own `Cargo.toml`.

```bash
cd <project-folder>
cargo run
```

This repository is mainly intended as a hands-on Rust learning workspace.
