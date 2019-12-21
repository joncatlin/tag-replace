use std::fs::File;
use std::collections::HashMap;
//use std::io::prelude::*;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    println!("Hello, Jon!");

    let mut empty = HashMap::new();
    let account_details = make_hashmap (&mut empty);
    println!("Hashmap={:?}", account_details);

    let replace_file = File::open("./data/Google.html")?;
    let mut buffer = String::new();
    let result = replace(replace_file,String::from("{{"), String::from("}}"), account_details, &mut buffer);

    Ok(())
}


// Tag a file and process it replacing any tags with the appropriate value from a hashmap
fn replace<'a> (in_file: File, start_tag_pattern: String, end_tag_pattern: String, replacement_values: &HashMap<String, String>, buffer: &'a mut String) -> &'a String {

    // Read the file until the start tag_pattern is found
    let mut cursor = io::Cursor::new(b"lorem-ipsum");
    let mut buf = vec![];
    
    // cursor is at 'l'
    let num_bytes = cursor.read_until(b'-', &mut buf)
        .expect("reading from cursor won't fail");
    assert_eq!(num_bytes, 6);
    assert_eq!(buf, b"lorem-");
    buf.clear();
    

    println!("Replacement_values={:?}", buffer);

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

