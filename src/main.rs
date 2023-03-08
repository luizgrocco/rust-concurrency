use core_affinity::set_for_current;
use std::sync::{Arc, Mutex};
use std::thread;

const MAX_SIZE: i64 = 100000000;

fn main() {
    let core_ids = core_affinity::get_core_ids().unwrap();

    let counter = Arc::new(Mutex::new(0i64));

    let handles = core_ids
        .into_iter()
        .map(|core_id| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                // Pin this thread to a single CPU core.
                let res = set_for_current(core_id);
                if res {
                    println!("Cpu {} successfully set affinity", core_id.id);
                    let mut count = counter.lock().unwrap();
                    let lowerbound = MAX_SIZE * core_id.id as i64;
                    let upperbound = core_id.id as i64 * MAX_SIZE + MAX_SIZE;
                    println!("lowerbound: {}, upperbound: {}", lowerbound, upperbound);
                    for i in lowerbound..upperbound {
                        *count += i;
                    }
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The total is {}", *counter.lock().unwrap());
}
