use std::{thread, time::Duration, sync::Arc};

fn main() {
    let handle = thread::spawn(|| {
        for n in 0..15 {
            println!("aaa {} ", n);
            thread::sleep(Duration::from_millis(10));
        }
    });

    let data = Arc::new(vec![1, 5 , 9, 10, 2]);
    // let data = vec![1, 5 , 9, 10, 2];
    let clone = data.clone();

    let handle2 = thread::spawn(move || {
        for n in clone.iter() {
            println!("bbbb {} ", n);
            thread::sleep(Duration::from_millis(10));
        }
    });
    
    println!("oooo");
    handle.join().unwrap();
    handle2.join().expect("woops");
}
