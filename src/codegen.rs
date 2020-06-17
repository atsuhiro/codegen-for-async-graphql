use std::env;
use std::fs;

mod parser;
mod template;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].clone();

    let schema = open_schema(&path);

    let doc = parser::parse(&schema);
    let object_types = parser::transform(&doc);
    template::generate_object_type(object_types);
}

fn open_schema(path: &String) -> String {
    fs::read_to_string(path).unwrap()
}
