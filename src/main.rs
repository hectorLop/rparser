pub mod rparser;

fn main() {

    if let Ok(file_content) = rparser::read_from_file("tests/10000_rows.csv") {
        println!("The CSV contains: {:} columns", file_content.content.len());
        let rows = match &file_content.content[0] {
                rparser::Column::Integer(column) => column.data.len(),
                rparser::Column::Float(column) => column.data.len(),
                rparser::Column::String(column) => column.data.len(),
            };
        println!("The CSV contains: {:} rows", rows);
        println!("Data types: {:?}", file_content.data_types);
    } else {
        panic!("Error parsing the file");
    }
}