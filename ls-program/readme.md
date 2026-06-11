## ls-program

A small Rust command-line program that behaves like a minimal `ls`.

It prints the command-line arguments it receives, then lists the contents of a directory. If no path is provided, it reads one from standard input. If the path is invalid, the program reports the error and exits with a non-zero status.

This is a simple exercise in using `std::env`, `std::fs::read_dir`, and basic error handling. 
