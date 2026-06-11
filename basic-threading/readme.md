## basic-threading

A small Rust example that demonstrates basic concurrency with threads.

The program spawns two worker threads: one prints a sequence of numbers with a short delay, while the other iterates over shared data stored in an `Arc` and prints each value. This makes it easy to see both threads running alongside the main thread.

At the end, the main thread waits for both workers to finish with `join`, so the program exits cleanly after all output has been printed.