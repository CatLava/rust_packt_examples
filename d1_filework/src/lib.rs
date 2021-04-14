// libraries and functions called in the code to compile Some
mod error;
pub use error::TransactionError;
use serde_derive::*;

//moved the error types to their own file
// struct for the transaction
#[derive(Deserialize,Serialize,Debug)]
pub struct Transaction{
    from : String,
    to: String,
    amount: u64,
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
pub fn get_first_transaction_for(fname: &str, uname: &str ) -> Option<Transaction> {
    // question mark at the end will know how to handle options
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
pub fn get_transactions_b(fname:&str)-> Result<Vec<Transaction>, TransactionError> {
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
