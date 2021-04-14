extern crate d1_filework;
use d1_filework::*;

fn main() -> Result<(), TransactionError> {
    println!("Hello, world!");
    let trans = get_transactions_b("transactions.json")?;
    for t in trans {
        println!("{:?}", t);
    }

    // ok_or converts option into a results
    let t = get_first_transaction_for("transactions.json", "Bob").ok_or("Could not get name")?;
    println!("Bobs first transaction {:?}", t);

    Ok(())
}
