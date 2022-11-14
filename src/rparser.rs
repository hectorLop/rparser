use csv;

use std::{error::Error, str::FromStr};

#[derive(Debug)]
pub enum DataTypes {
    String,
    Float,
    Integer
}

pub struct FileContent  {
    pub headers: Vec<String>,
    pub data_types: Vec<DataTypes>,
    pub content: Vec<Column>,
}

#[derive(Debug)]
pub enum Column {
    String(Col<Vec<String>>),
    Integer(Col<Vec<i64>>),
    Float(Col<Vec<f64>>),
}

#[derive(Debug)]
pub struct Col<T> {
    pub data: T
}

impl<T> Col<T> {
    fn new(value: T) -> Self {
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

pub fn read_from_file(path: &str) -> Result<FileContent, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let mut file_content: FileContent = FileContent {
        headers: Vec::new(),
        data_types: Vec::new(),
        content: Vec::new(),
    };

    file_content.headers = reader.headers().cloned().unwrap().deserialize(None)?;
    
    {
        let first_row = reader.records().next().unwrap()?;

        for value in first_row.into_iter() {
            if match_type::<i64>(value) {
                file_content.content.push(Column::Integer(Col::new(Vec::new())));
                file_content.data_types.push(DataTypes::Integer);
            } else if match_type::<f64>(value) {
                file_content.content.push(Column::Float(Col::new(Vec::new())));
                file_content.data_types.push(DataTypes::Float);
            } else {
                file_content.content.push(Column::String(Col::new(Vec::new())));
                file_content.data_types.push(DataTypes::String);
            }
        }
    }
    

    for record in reader.records() {
        match &record {
            Ok(row) => {
                for (i, field) in row.into_iter().enumerate() {
                    match &mut file_content.content[i] {
                        Column::Integer(column) => column.data.push(field.parse::<i64>()?),
                        Column::Float(column) => column.data.push(field.parse::<f64>()?),
                        Column::String(column) => column.data.push(field.parse::<String>()?),
                    }
                }
            },
            Err(_e) => (),
        };
    }

    Ok(file_content)
}

