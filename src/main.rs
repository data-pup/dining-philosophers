use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

fn main() {
    const N: usize = 10;

    // Spawn a few threads to increment a shared variable (non-atomically), and
    // let the main thread know once all increments are done.
    //
    // Here we're using an Arc to share memory among threads, and the data inside
    // the Arc is protected with a mutex.
    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = channel();
    let mut handles = vec![];
    for i in 0..N {
        let (data, tx) = (data.clone(), tx.clone());
        let new_handle = thread::spawn(move || {
            // The shared state can only be accessed once the lock is held.
            // Our non-atomic increment is safe because we're the only thread
            // which can access the shared state when the lock is held.
            //
            // We unwrap() the return value to assert that we are not expecting
            // threads to ever fail while holding the lock.
            let mut data = data.lock().unwrap();
            println!("We are in thread: {}\tData was: {}", i, *data);
            *data += 1;
            println!("We are in thread: {}\tData now is: {}", i, *data);
            // if *data == N {
            tx.send((*data)).unwrap();
            // }
            // the lock is unlocked here when `data` goes out of scope.
        });
        handles.push(new_handle);
    }

    let join_result: Result<(), _> = handles.into_iter().map(|handle| handle.join()).collect();

    // let res: u32 = rx.recv().unwrap();
    for res in rx {
        println!("Received: {:?}", res);
    }
}
