pub mod search {
    use std::collections::{HashMap, LinkedList};
    use std::{fs, path, fmt, io};
    use std::cmp::Reverse;

    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub struct Posting {
        pub id: u32,
        pub tf: u32,
    }

    impl fmt::Display for Posting {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Posting {{ id: {}, tf: {} }}", self.id, self.tf)
        }
    }

    pub fn build_index() -> Result<HashMap<String, LinkedList<Posting>>, &'static str>{
        let mut index: HashMap<String, LinkedList<Posting>> = HashMap::new();

        // parse documents directory
        let document_store_path = path::Path::new("documents");

        if document_store_path.is_dir() {
            for entry in fs::read_dir(&document_store_path).unwrap() {
                let mut tf_hashmap: HashMap<String, u32> = HashMap::new();
                let doc = entry.unwrap(); // Extract DirEntry from Result

                let file_name = doc.file_name(); // OsString
                let file_stem = file_name.to_string_lossy(); // Convert to string

                let id: u32 = file_stem.parse().expect("id should be in file name");

                let content = fs::read_to_string(doc.path()).unwrap(); // Use `doc.path()`

                let terms: Vec<&str> = content.split_whitespace().collect();

                let normalized_terms: Vec<String> = terms.iter()
                    .map(|t| t.trim().to_lowercase().trim_matches(&['.', ',']).to_string())
                    .collect();

                // add to Terminology
                for term in normalized_terms.iter() {
                    //store tf
                    let tf = tf_hashmap.entry(term.to_string()).or_insert(0);
                    *tf += 1;
                };

                // add to Index
                for ( t, tf ) in &tf_hashmap {
                    let posting = Posting { id, tf: tf.clone() };
                    let mut posting = LinkedList::from([posting]);
                    let postlist = index.entry(t.to_string()).or_insert(LinkedList::new());
                    postlist.append(&mut posting);
                }
            }
        } else {
            return Err("Document store could not be parsed: Wrong path provided.");
        }
        return Ok(index);
    }

    pub fn run(index: HashMap<String, LinkedList<Posting>>) {
        loop {
            let mut query = String::new(); 
            println!("Enter a query:\n");
            io::stdin().read_line(&mut query).expect("query should be read correctly.");
            let query = query.trim().to_string();
            match query.as_str() {
                "quit" => break,
                _ => {
                    search(&query, &index);
                }
            }
            
        }
        println!("Quitting...");
    }

    pub fn search (query: &String, index: &HashMap<String, LinkedList<Posting>>) {
        let q = query.to_string();
        if !index.contains_key(&q) {
            println!("No results for query '{}'.\n", q);
            return
        } else {
            let postings = index.get(&q).unwrap();
            let mut postings: Vec<&Posting> = postings.iter().collect();
            postings.sort_by_key(|posting| Reverse(posting.tf));
            for posting in postings {
                println!(" DOCUMENT {} ({} Match(es))", 
                    posting.id,
                    posting.tf
                );
                println!("{}", get_snippet(&posting.id));
                println!("-------------------------\n");
            }
        }
    }

    pub fn parse_document(id: &u32) -> String {
        // parse documents directory
        let path = "./documents/".to_string() + &id.to_string();

        let path = path::Path::new(&path);

        let content = fs::read_to_string(path).unwrap(); 

        content
    }

    pub fn parse_document_to_terms(id: &u32) -> Vec<String> {
        // parse documents directory
        let path = "./documents/".to_string() + &id.to_string();

        let path = path::Path::new(&path);

        let content = fs::read_to_string(path).unwrap(); 

        let terms: Vec<&str> = content.split_whitespace().collect();

        let normalized_terms: Vec<String> = terms.iter()
            .map(|t| t.trim().to_lowercase().trim_matches(&['.', ',']).to_string())
            .collect();

        normalized_terms
    }

    pub fn get_snippet(id: &u32) -> String {
        let snippet = parse_document(id);
        let dots = String::from("...");
        snippet[..80].to_string() + &dots
    }
}

#[cfg(test)]
mod tests {
    use crate::search::*;

    #[test]
    fn index_built_correctly(){
        let index = build_index();
        assert!(index.is_ok());
    }

    #[test]
    fn finds_something_for_query() {
        let query = String::from("to");
        let index = build_index().unwrap();
        crate::search::search(&query, &index);
        assert!(index.get(&query).unwrap().len() > 0);
    }
}