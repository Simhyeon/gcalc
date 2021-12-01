use std::error::Error;
use csv::WriterBuilder;
use prettytable::{Table, Row, Cell};

pub struct Formatter;

impl Formatter {
    pub fn to_csv_string (
        values :Vec<Vec<String>>,
        range: (usize,usize)
    ) -> Result<String, Box<dyn Error>> {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        let (min,max) = range;

        for (index, value) in values.iter().enumerate() {
            if index >= min && index <= max {
                wtr.write_record(value)?;
            }
        }

        let data = String::from_utf8(wtr.into_inner()?)?;

        Ok(data)
    }

    pub fn to_table(
        values :Vec<Vec<String>>,
        range: (usize,usize)
    ) -> Table {
        let mut table = Table::new();
        let (min,max) = range;

        for (index,row) in values.iter().enumerate() {
            if index >= min && index <= max {
                let row = row.iter().map(|v| Cell::new(&v)).collect();
                table.add_row(Row::new(row));
            }
        }
        table
    }
}
