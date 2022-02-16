use std::error::Error;
#[cfg(feature = "tabled")]
use tabled::{Table, Style};

use crate::{models::{Record, Qualficiation}, GcalcResult};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub(crate) struct QualFormatter;

impl QualFormatter {
    // pub fn to_csv_table( qual :Qualficiation) -> Result<String, Box<dyn Error>> {
    pub fn to_csv_table(qual :Qualficiation) -> GcalcResult<String> {
        let data = qual.join_as_csv();
        Ok(data)
    }

    #[cfg(feature = "tabled")]
    pub fn to_styled_table(qual :Qualficiation, style: Style) -> String {
        let table : Table = Table::new(vec![qual]).with(style);
        table.to_string()
    }
}

pub(crate) struct RecordFormatter;

impl RecordFormatter {
    pub fn to_raw_csv (
        values :&Vec<Record>,
        range: Option<(usize,usize)>
    ) -> Result<String, Box<dyn Error>> {
        let mut string_records = vec![];
        let (min,max) = if let Some((min,max)) = range {
            (min,max)
        } else {
            (0,values.len())
        };

        for (index, value) in values.iter().enumerate() {
            if index >= min && index <= max {
                string_records.push(value.join_as_csv());
            }
        }

        let data = string_records.join(LINE_ENDING);

        Ok(data)
    }

    #[cfg(feature = "tabled")]
    pub fn to_styled_table(
        values : &Vec<Record>,
        range: Option<(usize,usize)>,
        style: Style
    ) -> String {
        let table: Table;
        let (min,max) = if let Some((min,max)) = range {
            (min,max)
        } else { (0,values.len()) };

        //  Convert to format record
        let values: Vec<_> = values.iter().map(|s| FormatRecord::from_record(s)).collect();

        // If range is for whole values(records)
        // skip range check and create whole table from values

        use crate::models::FormatRecord;
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
