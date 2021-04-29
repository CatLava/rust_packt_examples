use std::time::Duration;
use std::sync::{Arc,Mutex};

// using arc and mutex

fn main(){
    with_arc();
}

fn with_arc(){
    // what if we want to communicate between the two threads
    //let mut m = 5;
    // Make m a string instead, int has the copy trait associated
    let m = Arc::new(Mutex::new(String::from("moving")));
    let m2 = m.clone();
    // to solve this moving issue, we bring in arc and other reference
    std::thread::spawn(move || {
        println!("This is a new thread");
        //m += 2;
        let mut s = m2.lock().unwrap();
        s.push_str("on this thread");
        println!("m = {}", s);
    });
    std::thread::sleep(Duration::from_millis(1000));
    println!("This is initial thread");
    let s = m.lock().unwrap();
    println!("now m = {}", s);

}
