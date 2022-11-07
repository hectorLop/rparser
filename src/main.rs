use csv;

use std::{error::Error};

#[derive(Debug)]
enum DataTypes {
    String,
    Float,
    Integer
}

struct FileContent<'a>  {
    headers: Vec<&'a str>,
    data_types: Vec<DataTypes>,
    content: Vec<Column>,
}

#[derive(Debug)]
enum Column {
    String(Col<Vec<String>>),
    Integer(Col<Vec<i64>>),
    Float(Col<Vec<f64>>),
}

#[derive(Debug)]
struct Col<T> {
    data: T
}

impl<T> Col<T> {
    fn new(mut value: T) -> Self {
        Self {data: value}
    }
}


pub fn match_integer(value: &str) -> bool {
    match value.parse::<i64>() {
        Ok(_n) => true,
        Err(_e) => false,
    }
}

pub fn match_float(value: &str) -> bool {
    match value.parse::<f64>() {
        Ok(_n) => true,
        Err(_e) => false,
    }
}

pub fn match_string(value: &str) -> bool {
    match value.parse::<String>() {
        Ok(_n) => true,
        Err(_e) => false,
    }
}

pub fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let headers: Vec<&str> = reader.headers().unwrap().deserialize(None)?;

    let mut data_types: Vec<DataTypes> = Vec::new();
    let mut content: Vec<Column> = Vec::new();
    
    let first_row = reader.into_records().skip(1).next().unwrap().unwrap();

    for value in first_row.into_iter() {
        if match_integer(value) {
            content.push(Column::Integer(Col::new(Vec::new())));
            data_types.push(DataTypes::Integer);
        } else if match_float(value) {
            content.push(Column::Float(Col::new(Vec::new())));
            data_types.push(DataTypes::Float);
        } else {
            content.push(Column::String(Col::new(Vec::new())));
            data_types.push(DataTypes::String);
        }
    }

    // println!("{:?}", headers);
    println!("Content: {:?}", content);
    println!("Data types: {:?}", data_types);

    // for row in reader.records().skip(1) {
    //     for field in row.unwrap().deserialize(None) {
            
    //     }
    // }
    // println!("There are {} rows", count);

    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("tests/10000_rows.csv") {
        eprintln!("{}", e);
    }
}