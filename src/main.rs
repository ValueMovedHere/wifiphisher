mod airodumpng_scan;
mod captive_portal;
mod csv;
mod records;

use std::path::Path;

use csv::records;

fn main() {
    let results = records(Path::new("/tmp/test-results/results-01.csv"));
    println!("{:#?}", results);
}
