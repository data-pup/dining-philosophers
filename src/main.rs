use std::thread;

// struct Spoon {
//     counter: u32,
// }

// impl Spoon {
// }

// struct Philosopher {
// }

// impl Philosopher {
// }

fn create_thread() -> thread::JoinHandle<()> {
    let v = vec![1, 2, 3];
    thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    })
}

fn main() {
    let handle = create_thread();
    handle.join().expect("Could not join");
}
