use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::collections::VecDeque;

/// Producer - Consumer example program. 
/// Both the prod and the cons are running with their own thread. They share a vector, protected by a big lock.
/// (to be perfected for better concurrency and performances)

struct SharedState {
    buffer: VecDeque<i32>,
    done: bool,
}

fn prod(shared: Arc<(Mutex<SharedState>, Condvar)>, size: usize) {
    for i in 0..size {
        let (lock, cvar) = &*shared; //recup arg
        let mut state = lock.lock().expect("mutex poisoned"); // recup tab + prise mutex
        state.buffer.push_back(i as i32); //update data
        println!("Produced: {}", i);
        cvar.notify_one();
    }
    //lock scope : the lock stays alive as long as the variable is still reachable 
    //(= hasn't been dropped)

    //change state of production
    let (lock, cvar) = &*shared;
    let mut state = lock.lock().expect("mutex poisoned");
    state.done = true;
    cvar.notify_all();
}

fn cons(shared: Arc<(Mutex<SharedState>, Condvar)>) {
    //loop over the vector if producer finished producing
    loop {
        let (lock, cvar) = &*shared;
        let mut state = lock.lock().expect("mutex poisoned");

        while state.buffer.is_empty() && !state.done { //fat condition : wait until production ended
            state = cvar.wait(state).expect("mutex poisoned");
        }

        if let Some(value) = state.buffer.pop_front(){
            println!("Consumed: {}", value);
        } else if state.done { // if producer started working again
            break;
        }
    }
}

fn main() {
    let size = 10;
    let shared = Arc::new((
        Mutex::new(SharedState {
            buffer: VecDeque::with_capacity(size),
            done: false,
        }),
        Condvar::new(),
    ));

    //producer
    let producer_shared = Arc::clone(&shared);
    let producer = thread::spawn(move || {
        prod(producer_shared, size);
    });

    //consumer
    let consumer_shared = Arc::clone(&shared);
    let consumer = thread::spawn(move || {
        cons(consumer_shared);
    });

    producer.join().expect("failed to join producer thread");
    consumer.join().expect("failed to join consumer thread");
}
