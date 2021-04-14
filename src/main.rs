extern crate d1_filework;
use d1_filework::*;
use failure::Error;

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let trans = get_transactions_b("transactions.json")?;
    for t in trans {
        println!("{:?}", t);
    }

    // ok_or converts option into a results
    let t = get_first_transaction_for("transactions.json", "Bobii");
    match t {
        Ok(v) => println!("Found Transaction: {:?}", v),
        Err(e) => println!("Error {}, Backtrace = : {}", e, e.backtrace())
    }
    // println!("Bobs first transaction {:?}", t);

    Ok(())
}
