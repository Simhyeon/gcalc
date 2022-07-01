use std::error::Error;
#[cfg(feature = "tabled")]
use tabled::{Style, Table};

use crate::{
    models::{Qualficiation, Record},
    GcalcResult,
};

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

pub(crate) struct QualFormatter;

impl QualFormatter {
    // pub fn to_csv_table( qual :Qualficiation) -> Result<String, Box<dyn Error>> {
    pub fn to_csv_table(qual: Qualficiation) -> GcalcResult<String> {
        let data = qual.join_as_csv();
        Ok(data)
    }

    #[cfg(feature = "tabled")]
    pub fn to_styled_table(qual: Qualficiation, style: Style) -> String {
        let table: Table = Table::new(vec![qual]).with(style);
        table.to_string()
    }
}

pub(crate) struct RecordFormatter;

impl RecordFormatter {
    pub fn to_raw_csv(
        records: &Vec<Record>,
        range: Option<(usize, usize)>,
    ) -> Result<String, Box<dyn Error>> {
        let mut string_records = vec!["count,probability,cost,constant,value".to_string()];
        let (min, max) = if let Some((min, max)) = range {
            (min, max)
        } else {
            (0, records.len())
        };
        for (index, record) in records.iter().enumerate() {
            if index >= min && index <= max {
                string_records.push(record.join_as_csv());
            }
        }

        let data = string_records.join(LINE_ENDING);

        Ok(data)
    }

    #[cfg(feature = "tabled")]
    pub fn to_styled_table(
        values: &Vec<Record>,
        range: Option<(usize, usize)>,
        style: Style,
    ) -> String {
        let (min, max) = if let Some((min, max)) = range {
            (min, max)
        } else {
            (0, values.len())
        };

        //  Convert to format record
        let values: Vec<_> = values.iter().map(FormatRecord::from_record).collect();

        // If range is for whole values(records)
        // skip range check and create whole table from values

        use crate::models::FormatRecord;
        let table = if (min, max) == (0, values.len()) {
            Table::new(values).with(style)
        } else {
            let mut scoped_records = vec![];
            for (index, row) in values.iter().enumerate() {
                if index >= min && index <= max {
                    scoped_records.push(row);
                }
            }
            Table::new(values).with(style)
        };

        table.to_string()
    }
}
