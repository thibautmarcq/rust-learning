# rust-learning

Basic repository for learning Rust through small independent projects.

Each folder contains a focused example built to practice a specific Rust concept or part of the ecosystem, without trying to be a full application.

## Contents

- [basic-threading](basic-threading/) - minimal threading example using threads, shared data via `Arc` and clean termination with `join`.
- [ls-program](ls-program/) - a small `ls`-style CLI working reading its path through the program arguments or stdin. Simply lists directory entries if the given path is correct. 
- [prod-cons](prod-cons/) - a producer-consumer threading example using a shared queue protected by `Mutex` and coordinated with `Condvar`.
