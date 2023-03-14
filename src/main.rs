#[allow(unused_variables)]
// use core_affinity::set_for_current;
use std::sync::{Arc, Mutex};
use std::thread;

// const MAX_SIZE: i64 = 100000000;
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    } // Potential data race!
}

fn main() {
    // let mut num = 0;

    // let unsafe_counter = &mut num as *mut i32;

    let correct_counter = Arc::new(Mutex::new(0));

    let correct_counter_copy1 = Arc::clone(&correct_counter);
    let correct_counter_copy2 = Arc::clone(&correct_counter);

    let handle1 = thread::spawn(move || {
        for _ in 0..10_000 {
            let mut count = correct_counter_copy1.lock().unwrap();
            *count += 1;
        }
    });

    let handle2 = thread::spawn(move || {
        for _ in 0..10_000 {
            let mut count = correct_counter_copy2.lock().unwrap();
            *count += 1;
        }
    });

    let handle3 = thread::spawn(move || {
        for _ in 0..=10_000 {
            add_to_counter(1);
        }
    });

    let handle4 = thread::spawn(move || {
        for _ in 0..=10_000 {
            add_to_counter(1);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    handle4.join().unwrap();

    println!();
    println!("------------RESULTS------------");
    unsafe {
        println!("The unsyncronized total is: {}", COUNTER);
    };
    println!(
        "The syncronized total is: {}",
        *correct_counter.lock().unwrap()
    );
    println!();

    // let core_ids = core_affinity::get_core_ids().unwrap();

    // let core_ids = core_ids[core_ids.len() / 2..core_ids.len() - 1].to_vec();

    // let counter = Arc::new(Mutex::new(0i64));

    // let handles = core_ids
    //     .into_iter()
    //     .map(|core_id| {
    //         let counter = Arc::clone(&counter);
    //         thread::spawn(move || {
    //             // Pin this thread to a single CPU core.
    //             let res = set_for_current(core_id);
    //             if res {
    //                 println!("Cpu {} successfully set affinity", core_id.id);
    //                 let mut count = counter.lock().unwrap();
    //                 let lowerbound = MAX_SIZE * core_id.id as i64;
    //                 let upperbound = core_id.id as i64 * MAX_SIZE + MAX_SIZE;
    //                 println!("lowerbound: {}, upperbound: {}", lowerbound, upperbound);
    //                 for i in lowerbound..upperbound {
    //                     *count += i;
    //                 }
    //             }
    //         })
    //     })
    //     .collect::<Vec<_>>();

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("The total is {}", *counter.lock().unwrap());
}
