use std::error::Error;
use csv::WriterBuilder;
use tabled::{Table, Style};

use crate::models::{Record, Qualficiation};

pub(crate) struct QualFormatter;

impl QualFormatter {
    pub fn to_csv_table( qual :Qualficiation) -> Result<String, Box<dyn Error>> {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        wtr.serialize(qual)?;
        let data = String::from_utf8(wtr.into_inner()?)?;

        Ok(data)
    }

    pub fn to_styled_table(qual :Qualficiation, style: Style) -> String {
        let table : Table = Table::new(vec![qual]).with(style);
        table.to_string()
    }
}

pub(crate) struct RecordFormatter;

impl RecordFormatter {
    pub fn to_raw_csv (
        values :Vec<Record>,
        range: Option<(usize,usize)>
    ) -> Result<String, Box<dyn Error>> {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        let (min,max) = if let Some((min,max)) = range {
            (min,max)
        } else {
            (0,values.len())
        };

        for (index, value) in values.iter().enumerate() {
            if index >= min && index <= max {
                wtr.serialize(value)?;
            }
        }

        let data = String::from_utf8(wtr.into_inner()?)?;

        Ok(data)
    }

    pub fn to_styled_table(
        values : Vec<Record>,
        range: Option<(usize,usize)>,
        style: Style
    ) -> String {
        let table: Table;
        let (min,max) = if let Some((min,max)) = range {
            (min,max)
        } else { (0,values.len()) };

        // If range is for whole values(records)
        // skip range check and create whole table from values
        if (min, max) == (0, values.len()) {
            table = Table::new(values).with(style);
        } else {
            let mut scoped_records = vec![];
            for (index,row) in values.iter().enumerate() {
                if index >= min && index <= max {
                    scoped_records.push(row);
                }
            }
            table = Table::new(values).with(style);
        }

        table.to_string()
    }
}
