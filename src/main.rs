extern crate rand;

use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

struct Fork {
    counter: u32,
}

impl Fork {
    fn new() -> Fork {
        Fork { counter: 0 }
    }
}

struct Philosopher {
    name: String,
    left: Arc<Mutex<Fork>>,
    right: Arc<Mutex<Fork>>,
}

fn main() {
    // let _table = init_table();

    let num_philosophers = 3;

    let forks = vec![
        Arc::new(Mutex::new(Fork::new())),
        Arc::new(Mutex::new(Fork::new())),
    ];

    let mut handles = vec![];

    let philosophers = vec![
        Philosopher {
            name: "Sarte".to_string(),
            left: forks.get(0).unwrap().clone(),
            right: forks.get(1).unwrap().clone(),
        },
        Philosopher {
            name: "Plato".to_string(),
            // This code would work, because each grabs the fork in the same order.
            // left: forks.get(0).unwrap().clone(),
            // right: forks.get(1).unwrap().clone(),

            // This code deadlocks.
            left: forks.get(1).unwrap().clone(),
            right: forks.get(0).unwrap().clone(),
        },
    ];

    for p in philosophers {
        let newthread = thread::spawn(move || {
            let r: u32 = rand::random();

            let ten_millis = time::Duration::from_millis((r % 100) as u64);
            for j in 0..5 {
                thread::sleep(ten_millis);
                let mut fork1 = p.left.lock().unwrap();
                println!("{} has grabbed the fork to their left", p.name);
                thread::sleep(ten_millis);
                let mut fork2 = p.right.lock().unwrap();
                println!("{} has grabbed the fork to their right", p.name);

                println!("{} is taking their {}th bite!", p.name, j);
                thread::sleep(ten_millis);
            }
        });

        handles.push(newthread);
    }
    for handle in handles {
        handle.join();
    }

    println!("Program done!");
}
