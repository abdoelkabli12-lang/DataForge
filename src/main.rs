use std::error::Error;
use std::cmp::{Reverse};
use csv::{Reader, StringRecord};
use std::fs::File;
use std::collections::{BinaryHeap, HashMap};
fn main() -> Result<(), Box<dyn Error>> {
    let filename: &str = "data/raw/online_retail_II.csv";
    let check = analyze_duplicate_pairs(filename);
    return check
}

fn analyze_duplicate_pairs(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut pairs_map: HashMap<(String, String), usize> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(usize, (String, String))>> = BinaryHeap::new();
    let file = File::open(filename)?;
    let mut rdr = Reader::from_reader(file);

    for result in rdr.records(){
        let record: StringRecord = result?;
        let invoice = record.get(0).expect("Missing column").to_string();
        let stockcode = record.get(1).expect("Missing column").to_string();
        let key_pair = (invoice, stockcode);
        *pairs_map.entry(key_pair).or_insert(0) += 1;
    }
    
    // let mut sorted: Vec<_> = pairs_map.into_iter().collect();
    let swapped_map = pairs_map.into_iter().map(|(k, v)| (v, k));
    for elem in swapped_map {
        if heap.len() < 5 {
            heap.push(Reverse(elem));
        } else {
            let head_val = heap.peek().unwrap();
            let count = head_val.0.0;
            if elem.0 >= count {
                heap.pop();
                heap.push(Reverse(elem));
            }
        }
    }

    while !heap.is_empty() {
        let large_val = heap.pop();
        let val = large_val.unwrap();
        let (counter, pair) = val.0;
        let (invoice, stockcode) = pair;
        println!("The pair {}, {} has occured {} times", invoice, stockcode, counter);
    }
    
    Ok(())
}
