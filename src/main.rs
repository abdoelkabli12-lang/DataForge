use std::error::Error;
use csv::{Reader, StringRecord};
use std::fs::File;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let filename: &str = "data/raw/malaga/listings.csv";
    let check = check_duplicate_ids(filename);
    return check
}

fn check_duplicate_ids(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut coll = HashSet::new();
    let file: File = File::open(filename)?;
    let mut rdr: Reader<File> = Reader::from_reader(file);

    for result in rdr.records() {
        let record: StringRecord = result?;
        let id: String = record.get(0).expect("Column Missing").trim().to_string();
        if coll.contains(&id) {
            println!("Found a duplicate")
        } else {
            coll.insert(id);
        }
        
    }

    Ok(())
}