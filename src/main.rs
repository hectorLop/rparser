use csv;

use std::error::Error;


fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let headers = reader.headers()?;
    println!("Headers: {:?}", headers);

    let mut count = 0;

    for row in reader.records() {
        count += 1;
    }
    println!("There are {} rows", count);

    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("src/tests/10000_rows.csv") {
        eprintln!("{}", e);
    }
}