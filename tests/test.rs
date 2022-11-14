pub use rparser;

#[cfg(test)]
mod tests {
    #[test]
    fn test_read_from_file() {
        if let Ok(file_content) = rparser::rparser::read_from_file("tests/10000_rows.csv") {
            assert_eq!(file_content.content.len(), 5);
            let rows = match &file_content.content[0] {
                rparser::rparser::Column::Integer(column) => column.data.len(),
                rparser::rparser::Column::Float(column) => column.data.len(),
                rparser::rparser::Column::String(column) => column.data.len(),
            };
            assert_eq!(rows, 9999);
        }
    }
}