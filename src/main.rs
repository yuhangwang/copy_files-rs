extern crate serde_json;
use std::fs;


struct Params {
    sources: Vec<String>,
    targets: Vec<String>,
}

fn main() {
    let data = r#"{
        sources: 
    }"#;
    let x = fs::copy("foo.txt", "bar.txt").unwrap();
}
