use std::fs::File;
use std::collections::HashMap;
//use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Hello, Jon!");

    let mut empty = HashMap::new();
    let account_details = make_hashmap (&mut empty);
    println!("Hashmap={:?}", account_details);

    let replace_file = File::open("./data/Google.html")?;
    let mut buffer = String::new();
    let result = replace(replace_file,String::from("pattern"), account_details, &mut buffer);
    println!("Buffer={:?}", buffer);

    Ok(())
}


// Tag a file and process it replacing any tags with the appropriate value from a hashmap
fn replace<'a> (in_file: File, tag_pattern: String, values: &HashMap<String, String>, buffer: &'a mut String) -> &'a String {
    buffer.push_str("Rubbish");
    buffer
}


// Initialize a hashmap containing some basic data for an account
fn make_hashmap (account_details: &mut HashMap<String, String>) -> &HashMap<String, String> {
    // Initialize the map with some test data
    account_details.insert("ACC#".to_string(),          "1".to_string());
    account_details.insert("FIRST_NAME".to_string(),    "2".to_string());
    account_details.insert("LAST_NAME".to_string(),     "3".to_string());
    account_details.insert("ADDR_1".to_string(),        "4".to_string());
    account_details.insert("ADDR_2".to_string(),        "5".to_string());
    account_details.insert("ADDR_3".to_string(),        "6".to_string());
    account_details.insert("ZIP_CODE".to_string(),      "7".to_string());

    account_details
}

