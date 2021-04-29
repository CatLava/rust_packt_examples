// This will count references and point to those objects.
// reference Count is what it is referred to.
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct WithLife<'a>{
    s:&'a String,

}

//using a reference count to make this function work
#[derive(Debug)]
pub struct NoLife{
    // generic type that acts as a reference
    // RefCell will allow for alteration to objects
    s:Rc<RefCell<String>>,
}

fn main() -> Result<(), std::io::Error>{
//    let (l1, l2) = make_with_life("test_data/v3_data.txt")?;
    let (n1, n2) = make_no_life("test_data/v3_data")?;

    // alterting the reference, mutable
    let mut s = n1.s.borrow_mut();
    s.push_str("What if string was bigger?");
    println!("n1 = {:?}", n1);
    println!("n2 = {:?}", n2);

    // s is a guard in itself, tells it it cna be dropped
    println!("s == {}", s);
    //drop(s)
    Ok(())
}


/*
// using lifetimes for this function
fn make_with_life<'a>(fname:&str)-> Result<(WithLife<'a>, WithLife<'a>),
std::io::Error> {
    let s = std::fs::read_to_string(fname)?;
    Ok((WithLife{s: &s}, WithLife{s: &s}))
}
*/
// no garbage collect in Rust, Rc acts as one
// not using a life time
fn make_no_life(fname: &str)->Result<(NoLife,NoLife),std::io::Error>{
    let s = std::fs::read_to_string(fname)?;
    // This will do a reference count on s
    let r = Rc::new(RefCell::new(s));
    // one needs to be a clone to inrement the reference count
    Ok((NoLife{s: r.clone()}, NoLife{s: r}))
}
