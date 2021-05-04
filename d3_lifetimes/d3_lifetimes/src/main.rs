// can attach llifetimes to objects
#[derive(Debug)]
pub struct StringHolder<'a>{
    s: &'a str,
    t: &'a str
}
fn main() {
    println!("Hello, world!");

    let mut s = make_str(7);
    //s.push_str("people");
    s = to_people(s);
    // this function takes in a mutable, changes s
    to_frogs(&mut s);
    // cannot mutate something while something else is borrowing it
    let p = part(&s);

    //s.push_str("p");

    println!("{}", p);
    println!("{}", s);

    let tog = two_strings(p, &s);
    println!("{:?}",tog)
}


fn to_people(mut s:String)-> String{
    s.push_str(" people");
    s
}

fn to_frogs(s:&mut String){
    s.push_str(" frogs");
}

// Need a lifetime on a local setup
// need to ask for memory from the Heap, need to free it
fn make_str(n:i32)-> String {
    format!("hello {}", n)
}

// will not return actively altered String
// memory garuntees in Rust
// we will attach a lifetime to this function
fn part<'a>(s: &'a str) -> &'a str {
    if s.len() > 4 {
        &s[0..4]
    } else {
        s
    }
}

pub fn two_strings<'a>(s:&'a str,t:&'a str)-> StringHolder<'a>{
    StringHolder {s, t}
}
