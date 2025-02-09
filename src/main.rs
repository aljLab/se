use se::search;
use std::collections::{HashMap, LinkedList};

fn main() {
    println!("Indexing ...");
    let index: HashMap<String, LinkedList<search::Posting>> = search::build_index()
        .unwrap();
    //call run function
    search::run(index);
}
