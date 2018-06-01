extern crate rand;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

struct Fork();

impl Fork {
    fn new() -> Fork {
        Fork {}
    }
}

struct Philosopher {
    name: String,
    left: Arc<Mutex<Fork>>,
    right: Arc<Mutex<Fork>>,
}

fn main() {
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
            // ears:
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

            let delay_time = time::Duration::from_millis((r % 100) as u64);
            let mut j = 0;
            while j < 5 {
                thread::sleep(delay_time);
                let mut _fork1 = p.left.lock().unwrap();
                println!("{} has grabbed the fork to their left", p.name);

                thread::sleep(delay_time);
                let _fork2 = match p.right.try_lock() {
                    Ok(x) => x,
                    Err(_) => {
                        println!("{} is putting their fork back down", p.name);
                        continue;
                    }
                };

                println!("{} has grabbed the fork to their right", p.name);
                println!("{} is taking their {}th bite!", p.name, j);
                thread::sleep(delay_time);
                j += 1;
            }
        });

        handles.push(newthread);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Program done!");
}
