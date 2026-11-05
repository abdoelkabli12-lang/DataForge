use std::error::Error;
use std::cmp::{Reverse, min};
use csv::{Reader, StringRecord};
use std::fs::File;
use std::collections::{HashMap};
fn main() -> Result<(), Box<dyn Error>> {
    let filename: &str = "data/raw/online_retail_II.csv";
    let check = analyze_duplicate_pairs(filename);
    return check
}

fn analyze_duplicate_pairs(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut pairs_map: HashMap<(String, String), usize> = HashMap::new();
    let file: File = File::open(filename)?;
    let mut rdr: Reader<File> = Reader::from_reader(file);

    for result in rdr.records(){
        let record: StringRecord = result?;
        let invoice: String = record.get(0).expect("Missing column").to_string();
        let stockcode: String = record.get(1).expect("Missing column").to_string();
        let key_pair: (String, String) = (invoice, stockcode);
        if pairs_map.contains_key(&key_pair) {
            *pairs_map.entry(key_pair).or_default() += 1;
        } else {
            pairs_map.insert(key_pair, 1);
        }
    }

    let mut sorted: Vec<_> = pairs_map.iter().collect();

    sorted.sort_by_key(|v| Reverse(*v.1));


    let end: usize = min(sorted.len(), 5);
    for (pair, counter) in &sorted[0..end] {
        let (invoice, stockcode) = *pair;
        println!("The pair {}, {} has occured {} times", invoice, stockcode, counter);
    }
    


    
    Ok(())
}
