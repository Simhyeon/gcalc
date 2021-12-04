use std::collections::HashMap;
use std::path::{Path, PathBuf};
use csv::Reader;

use crate::{GcalcResult, GcalcError};
use crate::formatter::Formatter;
use crate::utils;

const bonus_prob: usize = 1;
const basic_prob: usize = 2;
// const basic_cost      : usize = 3;

// TODO 
// Csv file
pub struct Calculator {
    start_probability: f32,
    count: usize,
    format: TableFormat,
    csv_file: Option<PathBuf>,
    header_map: Option<HashMap<&'static str, String>>,
    prob_precision: Option<usize>,
    prob_type: ProbType,
    // Which behaviour to take when csv rows ends
    // CSVreturn : ReturnType,
    // -> Eary return and break loop
    // -> Respect previous value
    // -> Panic
}

pub enum TableFormat {
    Console,
    Csv,
}

pub enum ProbType {
    Percentage,
    Float,
}

impl Calculator {
    // <BUILDER>
    // Constructor methods
    pub fn new(start_probability: f32, count: usize) -> GcalcResult<Self> {
        if start_probability >= 1.0f32 {
            return Err(GcalcError::InvalidArgument(format!("Given probabilty of {} which should not be bigger than 1.0", start_probability)));
        }

        Ok(Self {
            start_probability,
            count,
            csv_file : None,
            header_map: None,
            format: TableFormat::Csv,
            prob_precision: None,
            prob_type: ProbType::Float,
        })
    }

    pub fn table_format(mut self, format: TableFormat) -> Self {
        self.format = format;
        self
    }

    pub fn prob_type(mut self, prob_type: ProbType) -> Self {
        self.prob_type = prob_type; 
        self
    }

    pub fn precision(mut self, precision: usize) -> GcalcResult<Self> {
        self.prob_precision.replace(precision);
        Ok(self)
    }

    pub fn csv_file(mut self, path: impl AsRef<Path>) -> GcalcResult<Self> {
        self.csv_file.replace(path.as_ref().to_owned());
        Ok(self)
    }

    // </BUILDER>

    // <PROCESSING>
    pub fn print_range(&self, range: Option<(usize, usize)>) -> GcalcResult<()> {
        let (min,max) = range.unwrap_or((0,self.count));
        if max > self.count {
            return Err(GcalcError::InvalidArgument(format!("Given max index {} is bigger than total count : {}",max,self.count)));
        }
        
        // Total csv records iterator
        let mut csv_records = if let Some(file) = &self.csv_file {
            Some(Reader::from_path(file).expect("Failed to read csv from given file").into_records())
        } else { None };
        
        let mut records : Vec<Vec<String>> = Vec::new();
        let fail_prob = 1f32 - self.start_probability;
        let mut last_sum = 0f32;
        for index in 0..self.count {
            let mut constant: f32 = 0f32;
            if let Some(csv) = &mut csv_records {
                match csv.next() {
                    Some(row) => {
                        constant = row.expect("Failed to parse row")
                            .iter()
                            .collect::<Vec<&str>>()[bonus_prob]
                            .parse()
                            .expect("Failed to parse")
                    }
                    None => { 
                        return Err(GcalcError::CsvError(format!("Empty row in index: {}", index))); 
                    }
                }
            } 
            
            let prob = self.get_gsec_single(fail_prob, &mut last_sum, index, constant)?;
            records.push(vec![(index + 1).to_string(), prob]);
        }

        self.print_table(vec!["count", "probabilty"],records, (min,max))?;
        Ok(())
    }

    pub fn print_until(&self, target_probability: f32) -> GcalcResult<()> {
        if target_probability >= 1.0f32 {
            return Err(GcalcError::InvalidArgument(format!("Given probabilty {} is bigger than 1.0", target_probability))); }

        let mut records : Vec<Vec<String>> = Vec::new();
        let fail_prob = 1f32 - self.start_probability;
        let mut last_sum = 0f32;
        let mut index = 0;
        while last_sum < target_probability {
            let prob = self.get_gsec_single(fail_prob, &mut last_sum, index, 0.0)?;
            records.push(vec![(index + 1).to_string(), prob]);
            index = index + 1;
        }

        // Caculate once more
        // NOTE
        // Don't have to add index becuase index is already added at the final while loop
        let prob = self.get_gsec_single(fail_prob, &mut last_sum, index, 0.0)?;
        records.push(vec![(index + 1).to_string(), prob]);

        self.print_table(vec!["count", "probabilty"],records, (0,index))?;
        Ok(())
    }

    pub fn print_required(&self, target_probability: f32 ,cost: Option<f32>) -> GcalcResult<()> {
        if target_probability >= 1.0f32 {
            return Err(GcalcError::InvalidArgument(format!("Given probabilty {} is bigger than 1.0", target_probability)));
        }

        let mut records : Vec<Vec<String>> = Vec::new();
        let fail_prob = 1f32 - self.start_probability;
        let mut last_sum = 0f32;
        let mut index = 0;
        while last_sum < target_probability {
            let prob = self.get_gsec_single(fail_prob, &mut last_sum, index, 0.0)?;
            records.push(vec![(index + 1).to_string(), prob]);
            index = index + 1;
        }
        let total_cost = utils::float_to_string(index as f32 * cost.unwrap_or(0f32), &self.prob_precision);
        let values = vec![vec![index.to_string(), total_cost]];

        self.print_table(vec!["count", "cost"],values, (0,index))?;

        Ok(())
    }

    // </PROCESSING>


    // <INTERNAL>
    /// Geometric sequence single take
    fn get_gsec_single(&self,fail_prob: f32, last_sum: &mut f32 ,index: usize, constant: f32) -> GcalcResult<String> {
        utils::prob_sanity_check(fail_prob)?;
        utils::prob_sanity_check(constant)?;
        let current_success = self.start_probability * ( fail_prob.powi(index as i32) );
        *last_sum = (*last_sum + current_success + constant).min(1.0f32);
        let prob = utils::get_prob_as_type(*last_sum, &self.prob_type, &self.prob_precision);
        Ok(prob)
    }

    fn print_table(
        &self,
        headers: Vec<impl AsRef<str>>,
        mut values :Vec<Vec<String>>,
        range: (usize, usize)
    ) -> GcalcResult<()> {
        let headers = headers.into_iter().map(|s| s.as_ref().to_owned()).collect();
        values.insert(0, headers);

        match self.format {
            TableFormat::Csv => {
                match Formatter::to_csv_string(values, range) {
                    Ok(csv) => println!("{}", csv),
                    Err(err) => return Err(GcalcError::FormatFail(err)),
                }
            }
            TableFormat::Console => {
                Formatter::to_table(values, range).printstd();
            }
        }
        Ok(())
    }
    // </INTERNAL>
}
