use csv;

use std::{error::Error, str::FromStr};

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

pub fn match_type<T>(value: &str) -> bool
where
    T: FromStr 
{
    match value.parse::<T>() {
        Ok(_n) => true,
        Err(_e) => false,
    }
}

pub fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let headers: Vec<&str> = reader.headers().unwrap().deserialize(None)?;

    let mut data_types: Vec<&DataTypes> = Vec::new();
    let mut content: Vec<Column> = Vec::new();
    
    let first_row = &reader.records().next().unwrap()?;

    for value in first_row.into_iter() {
        if match_type::<i64>(value) {
            content.push(Column::Integer(Col::new(Vec::new())));
            data_types.push(&DataTypes::Integer);
        } else if match_type::<f64>(value) {
            content.push(Column::Float(Col::new(Vec::new())));
            data_types.push(&DataTypes::Float);
        } else {
            content.push(Column::String(Col::new(Vec::new())));
            data_types.push(&DataTypes::String);
        }
    }

    for record in reader.records() {
        match &record {
            Ok(row) => {
                for (i, field) in row.into_iter().enumerate() {
                    match &mut content[i] {
                        Column::Integer(column) => column.data.push(field.parse::<i64>()?),
                        Column::Float(column) => column.data.push(field.parse::<f64>()?),
                        Column::String(column) => column.data.push(field.parse::<String>()?),
                    }
                }
            },
            Err(_e) => (),
        };
    }

    // let file_content: FileContent = FileContent {
    //     headers: headers,
    //     data_types: data_types,
    //     content: content,
    // };

    // println!("{:?}", headers);
    println!("The CSV contains: {:} columns", content.len());
    for column in content.into_iter() {
        match &column {
            Column::Integer(column) => println!("Integer column, with {} elements", column.data.len()),
            Column::Float(column) => println!("Float column, with {} elements", column.data.len()),
            Column::String(column) => println!("String column, with {} elements", column.data.len()),
        }
    }
    println!("Data types: {:?}", data_types);

    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("tests/10000_rows.csv") {
        eprintln!("{}", e);
    }
}