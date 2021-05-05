
// this is a prepackaged iterator, 
use rayon::prelude::*;
use rayon::join;
use std::time::Duration;
// example of rust knowing what to occur on the compile system
pub fn on_each<T, F>(v:&mut [T] , f:F)
// calling this function and ability to send on multiple threads
// Copy allows for move funciton to happen on that function
    where F:Fn(&mut T) + Send + Copy + Sync,
    T: Send,
 {
    match v.len() {
        0=>return,
        n if n < 4 => {
            for i in v {
                f(i)
            }
        }
        n =>  {
            // split into two mutable pointers
            let (v1, v2) = v.split_at_mut(n/2);
            // once one thread completes, pass it to the next
            join(||on_each(v1,f), ||on_each(v2,f));

            }
        }
    }

fn main(){
    let mut v = Vec::with_capacity(100);
    for i in 0..100 {
        v.push(i);
    }

    on_each(&mut v , |n| {
        println!("doing = {}", n);
        std::thread::sleep(Duration::from_millis(600));
    });
    println!("result {:?}",v);

    // this is using rayon prelude for dissection and implementation
    v.par_iter_mut().for_each(|n| {
        println!("now doing {}", n);
        *n *= 10;
    });
    println!("result = {:?}", v)
}
