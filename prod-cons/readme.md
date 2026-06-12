## prod-cons

A small Rust example that demonstrates the classic producer-consumer pattern with threads.

The program starts one producer thread and one consumer thread that share a buffered queue protected by a `Mutex` and coordinated with a `Condvar`. The producer pushes a fixed range of values into the queue, signals the consumer as new items become available, and marks the work as finished when it is done.

The consumer waits when the buffer is empty, wakes up when the producer adds more data, and keeps draining values until production is complete. This makes the example a simple introduction to shared state, blocking synchronization, and thread coordination in Rust.
