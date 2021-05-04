use std::time::Duration;
use std::sync::{Arc,Mutex};

// using arc and mutex

fn main(){
    with_arc();
    with_channels();
}

pub fn with_arc(){
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

//channels will wrap mutlipl mutexs here
pub fn with_channels(){
    // mpsc = multi producer, multi sender
    let (ch_s, ch_r) = std::sync::mpsc::channel::<Box<Fn(&mut String) + Send>>();

    std::thread::spawn(move || {
        let mut hidden = String::new();
        loop {
            match ch_r.recv() {
                Ok(f)=>{
                    f(&mut hidden);
                    println!("This is hidden {}", hidden);

                }
                Err(_)=>{
                    println!("Completed!");
                    return;
                }
            }
        }
    });
    ch_s.send(Box::new(|s:&mut String| {
        s.push_str("Hello");
    })).unwrap();
    ch_s.send(Box::new(|s:&mut String| {
        s.push_str(" World");
    })).unwrap();

    drop(ch_s);

    std::thread::sleep(Duration::from_millis(1000));
}
