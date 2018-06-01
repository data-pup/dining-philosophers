extern crate rand;

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;
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
    left: Option<u32>, // FIXUP: What type should this be?
    right: Option<u32>, // FIXUP: What type should this be?
}

impl Philosopher {
    fn do_work() {
        unimplemented!();
    }
}

struct Table {
    forks: Vec<Fork>,
    ppl: Vec<Philosopher>,
}

fn init_table() -> Table {
    Table {
        forks: vec![
            Fork::new(),
            Fork::new(),
        ],
        ppl: vec![
            Philosopher {
                name: "Sarte".to_string(),
                left: None,
                right: None,
            },
            Philosopher {
                name: "Plato".to_string(),
                left: None,
                right: None,
            },
        ],
    }
}


fn main() {
    let _table = init_table();

    let num_philosophers = 3;
    let fork1 = Arc::new(Mutex::new(Fork::new()));
    let fork2 = Arc::new(Mutex::new(Fork::new()));
    let mut handles = vec![];

    for i in 0..num_philosophers {
      let fork1 = fork1.clone();
      let fork2 = fork2.clone();
      let newthread = thread::spawn(move || {
          let r: u32 = rand::random();

          let ten_millis = time::Duration::from_millis((r % 100u32) as u64);
          for j in 0..10 {
            thread::sleep(ten_millis);
            let mut fork1 = fork1.lock().unwrap();
            thread::sleep(ten_millis);
            let mut fork2 = fork2.lock().unwrap();
            thread::sleep(ten_millis);

            println!("Philosopher {} has claimed the forks on the {}th iteration!", i, j);
          }
      });
      handles.push(newthread);
    }
    for handle in handles {
        handle.join();
    }

    println!("Program done!");
}
