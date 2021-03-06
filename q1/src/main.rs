use std::thread;
use std::sync::{Mutex, Arc};
use thread::sleep_ms;

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        sleep_ms(1000); // Applies a 'simultaneity fudge factor'
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        sleep_ms(1000);

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),

    ]});

    let philosophers = vec![
        Philosopher::new("Number 1 ", 0, 1),
        Philosopher::new("Number 2 ", 1, 2),
        Philosopher::new("Number 3 ", 0, 2),

    ];

    let handles: Vec<_> = philosophers.into_iter().map( |philosopher| {
        let table = table.clone();

        thread::spawn(move || {
            philosopher.eat(&table);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
