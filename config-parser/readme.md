## config-parser

A small Rust command-line program that parses a configuration file.

The program asks for a file path, reads `nom`, `port`, and `debug` values, and reports parsing errors through a custom `ConfigError` type. Unknown keys are ignored, while missing or invalid required fields produce an error.

This is an exercise in file I/O, `Result`-based error handling, parsing, and organizing code into modules.
