use std::thread;
use std::sync::{Arc, Mutex};

use std::collections::VecDeque;

fn
main()
{
    let i: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let i_inc = i.clone();
    let i_dec = i.clone();

    let j_inc = thread::spawn(move || {
        for _ in 0..1_000_000
        {
            let mut i_inc = i_inc.lock().unwrap();
            *i_inc += 1;
        }
    });

    let j_dec = thread::spawn(move || {
        for _ in 0..1_000_000
        {
            let mut i_dec = i_dec.lock().unwrap();
            *i_dec -= 1;
        }
    });

    j_inc.join().unwrap();
    j_dec.join().unwrap();

    println!("Total num is {}", *i.lock().unwrap());

    println!()
}
