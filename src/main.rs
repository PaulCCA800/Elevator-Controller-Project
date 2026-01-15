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

    println!("Start Part 2.");

    //let vec = VecDeque::new();
    //vec.
    let i: Arc<Mutex<VecDeque<i32>>> = Arc::new(Mutex::new(VecDeque::new()));

    let p_out = i.clone();
    let c_out = i.clone();

    let p_inc = thread::spawn(move || {
        for i in 0i32..=30i32
        {
            let mut p_out = p_out.lock().unwrap();
            println!("Push {i}...");
            p_out.push_front(i);
        }
    });

    let c_inc = thread::spawn(move || {
        loop
        {
            let mut c_out = c_out.lock().unwrap();
            match c_out.pop_back()
            {
                None => {
                    println!("Pop Failed...");
                },
                Some(i) => {
                    println!("Pop {i}...");
                    if i == 30
                    {
                        break;
                    }
                },
            };
        }
    });

    p_inc.join().unwrap();
    c_inc.join().unwrap();




}
