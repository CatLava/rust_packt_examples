use serde_derive::*;

#[derive(Debug)]
pub enum TransactionError{
    LoadError(std::io::Error),
    ParseError(serde_json::Error),
    // static method ensures this is live throughout lifetime of program
    Mess(&'static str),
}

impl From<std::io::Error> for TransactionError {
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}

impl From<&'static str> for TransactionError {
    fn from(e: &'static str)-> Self{
        TransactionError::Mess(e)
    }
}
// struct for the transaction
#[derive(Deserialize,Serialize,Debug)]
pub struct Transaction{
    from : String,
    to: String,
    amount: u64,
}

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


// valid transaction function with error handling
pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, String> {
    //Err("No Trans".to_string());
    let s = match std::fs::read_to_string(fname){
        Ok(v)=>v,
        Err(e)=>return Err(e.to_string()),
    };
    let t: Vec<Transaction> = match  serde_json::from_str(&s) {
        Ok(v)=>v,
        Err(e)=>return Err(e.to_string()),
    };
    Ok(t)
}

// using an Option in this function, similar to the result Enum
fn get_first_transaction_for(fname: &str, uname: &str ) -> Option<Transaction> {
    let trans = get_transactions_b(fname).ok()?;
    for t in trans {
        if t.from == uname {
            return Some(t);
        }
    }
    None
}
// This does exact the same thing as above, mapping and error handling
// Converting to an option, what is an option?
fn get_transactions_b(fname:&str)-> Result<Vec<Transaction>, TransactionError> {
    /*std::fs::read_to_string(fname)
        .map_err(|e| e.into())
        .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.into()))

    Ok(
        match serde_json::from_str(&match std::fd::read_to_string(fname){
             Ok(v)=>v,
             Err(e)=>return Err(e.into()),
         }) {
             Ok(v) => v,
             Err(e)=> return Err(e.into()),
         },

         */
    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
}
