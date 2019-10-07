use keepass::{Database, Node};
use keepass::result::{Result};
use std::{fs::File, path::Path};
use std::collections::HashMap;


pub fn import_vault(path: &Path, password: String) -> Result<HashMap<String,String>> {
    let db = Database::open(
        &mut File::open(path)?,
        Some(&*password),
        None
    )?;

    let mut seeds = HashMap::new();
    for node in &db.root {
        if let Node::EntryNode(e) = node {
            seeds.insert(
                e.get_title().unwrap().to_string(),
                e.get("Seed").unwrap().to_string()
            );
        }
    }

    Ok(seeds)
}

fn main() {
    let path = std::path::Path::new("TEST.kdbx");
    let password = String::from("fjFKdkAlrZln");
    let s = import_vault(&path, password);
    println!("{:#?} DB Opened", s);
}