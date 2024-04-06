use scrounger::Scrounger;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut scrounger = Scrounger::new(args[1].to_owned());
    scrounger.search();
    scrounger.search_files();
}
