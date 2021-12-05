use std::collections::HashMap;
use std::path::{Path, PathBuf};
use csv::{Reader, StringRecordsIntoIter};

use crate::{GcalcResult, GcalcError};
use crate::formatter::Formatter;
use crate::utils;

const count_index       : usize = 0;
const basic_prob_index  : usize = 1;
const bonus_prob_index  : usize = 2;
const basic_cost_index  : usize = 3;

// TODO 
// Csv file
pub struct Calculator {
    state: CalcState,
    count: usize,
    format: TableFormat,
    csv_file: Option<PathBuf>,
    header_map: Option<HashMap<&'static str, String>>,
    prob_precision: Option<usize>,
    budget: Option<f32>,
    target_probability : Option<f32>,
    prob_type: ProbType,
    // Which behaviour to take when csv rows ends
    // CSVreturn : ReturnType,
    // -> Eary return and break loop
    // -> Respect previous value
    // -> Panic
}

struct CalcState {
    pub probabilty: f32,
    pub constant: f32,
    pub cost: f32,
    pub calc_result: placeholder,
}


impl CalcState {
    pub fn new(probabilty: f32) -> Self {
        Self {
            probabilty,
            constant: 0.0,
            cost: 0.0,
            calc_result: placeholder::None,
        }
    }
}

enum placeholder {
    None,
}

pub enum TableFormat {
    Console,
    Csv,
}

impl TableFormat {
    pub fn from_str(string : &str) -> GcalcResult<Self> {
        match string.to_lowercase().as_str() {
            "console" => Ok(Self::Console),
            "csv" => Ok(Self::Csv),
            _ => Err(GcalcError::InvalidConversion(format!("{} is not a valid table format", string))),
        }
    }
}

pub enum ProbType {
    Percentage,
    Float,
}

impl ProbType {
    pub fn from_str(string : &str) -> GcalcResult<Self> {
        match string.to_lowercase().as_str() {
            "percentage" => Ok(Self::Percentage),
            "float" => Ok(Self::Float),
            _ => Err(GcalcError::InvalidConversion(format!("{} is not a valid table format", string))),
        }
    }
}

impl Calculator {
    // <BUILDER>
    // Constructor methods
    pub fn new(start_probability: f32, count: usize) -> GcalcResult<Self> {
        if start_probability >= 1.0f32 {
            return Err(GcalcError::InvalidArgument(format!("Given probabilty of {} which should not be bigger than 1.0", start_probability)));
        }

        Ok(Self {
            state : CalcState::new(start_probability),
            count,
            csv_file : None,
            header_map: None,
            format: TableFormat::Csv,
            prob_precision: None,
            target_probability: None,
            budget: None,
            prob_type: ProbType::Float,
        })
    }

    pub fn target_probability(mut self, target_probability: f32) -> Self {
        self.target_probability.replace(target_probability);
        self
    }
    
    pub fn budget(mut self, budget: f32) -> Self {
        self.budget.replace(budget);
        self
    }

    pub fn table_format(mut self, format: TableFormat) -> Self {
        self.format = format;
        self
    }

    pub fn prob_type(mut self, prob_type: ProbType) -> Self {
        self.prob_type = prob_type; 
        self
    }

    pub fn precision(mut self, precision: usize) -> Self {
        self.prob_precision.replace(precision);
        self
    }

    pub fn csv_file(mut self, path: impl AsRef<Path>) -> Self {
        self.csv_file.replace(path.as_ref().to_owned());
        self
    }

    // </BUILDER>
    //
    // <Setter>
    pub fn set_target_probability(&mut self, target_probability: f32)  {
        self.target_probability.replace(target_probability);
    }
    
    pub fn set_budget(&mut self, budget: f32) {
        self.budget.replace(budget);
    }

    pub fn set_table_format(&mut self, format: TableFormat) {
        self.format = format;
    }

    pub fn set_prob_type(&mut self, prob_type: ProbType) {
        self.prob_type = prob_type; 
    }

    pub fn set_precision(&mut self, precision: usize) {
        self.prob_precision.replace(precision);
    }

    pub fn set_csv_file(&mut self, path: impl AsRef<Path>) {
        self.csv_file.replace(path.as_ref().to_owned());
    }
    // </Setter>

    // <PROCESSING>
    pub fn print_range(&mut self, range: Option<(usize, usize)>) -> GcalcResult<()> {
        let (min,max) = range.unwrap_or((0,self.count));
        if max > self.count {
            return Err(GcalcError::InvalidArgument(format!("Given max index {} is bigger than total count : {}",max,self.count)));
        }
        let records = self.create_records()?;
        self.print_table(vec!["count", "probabilty"],records, (min,max))?;
        Ok(())
    }

    pub fn print_required(&self, target_probability: f32 ,cost: Option<f32>) -> GcalcResult<()> {
        if target_probability >= 1.0f32 {
            return Err(GcalcError::InvalidArgument(format!("Given probabilty {} is bigger than 1.0", target_probability)));
        }

        let mut records : Vec<Vec<String>> = Vec::new();
        let mut last_sum = 0f32;
        let mut index = 0;
        while last_sum < target_probability {
            let prob = self.get_gsec_prob(index)?;
            last_sum= last_sum + prob;
            let prob_str = utils::get_prob_as_type(last_sum, &self.prob_type, &self.prob_precision);
            records.push(vec![(index + 1).to_string(), prob_str]);
            index = index + 1;
        }

        let total_cost = utils::float_to_string(index as f32 * cost.unwrap_or(0f32), &self.prob_precision);
        let values = vec![vec![index.to_string(), total_cost]];

        self.print_table(vec!["count", "cost"],values, (0,index))?;

        Ok(())
    }

    /// Creat recors accroding to miscellaenous states
    fn create_records(&mut self) -> GcalcResult<Vec<Vec<String>>> {
        // Total csv records iterator
        let mut csv_records = if let Some(file) = &self.csv_file {
            Some(Reader::from_path(file).expect("Failed to read csv from given file").into_records())
        } else { None };

        let mut records : Vec<Vec<String>> = Vec::new();
        let mut last_sum = 0f32;
        let mut total_cost = 0f32;

        for index in 0..self.count {
            self.update_state_from_csv_file(&mut csv_records, index)?;
            
            let prob = self.get_gsec_prob(index)?;
            last_sum = (last_sum + prob).min(1.0f32);
            total_cost = total_cost + self.state.cost;

            let prob_str = utils::get_prob_as_type(last_sum, &self.prob_type, &self.prob_precision);
            records.push(vec![(index + 1).to_string(), prob_str]);
            
            // If current probabilty is bigger than target_probability break
            if let Some(target) = self.target_probability {
                if prob >= target {
                    break;
                }
            }

            // If currnet cost is bigger than budget break;
            if let Some(budget) = self.budget {
                if prob >= budget {
                    break;
                }
            }
        }

        Ok(records)
    }

    // </PROCESSING>


    // <INTERNAL>
    fn update_state_from_csv_file(&mut self, csv_reader :&mut Option<StringRecordsIntoIter<std::fs::File>>, index: usize) -> GcalcResult<()> {
        if let Some(csv) = csv_reader {
            match csv.next() {
                Some(row) => {
                    let row = row.expect("Failed to parse row"); // Temporary bound
                    let row = row.iter().collect::<Vec<&str>>();

                    if row.len() > bonus_prob_index {
                        self.state.constant = row[bonus_prob_index]
                            .parse()
                            .expect("Failed to parse");
                    }
                    if row.len() > basic_prob_index {
                        self.state.probabilty = row[basic_prob_index]
                            .parse()
                            .expect("Failed to parse");
                    }
                    if row.len() > basic_cost_index {
                        self.state.cost = row[basic_cost_index]
                            .parse()
                            .expect("Failed to parse");
                    }
                }
                None => { 
                    return Err(GcalcError::CsvError(format!("Empty row in index: {}", index))); 
                }
            }
        } 
        Ok(())
    }

    /// Geometric sequence single take
    fn get_gsec_prob(&self,index: usize) -> GcalcResult<f32> {
        utils::prob_sanity_check(self.state.constant)?;
        let fail_prob = 1f32 - self.state.probabilty;
        let current_success = self.state.probabilty * ( fail_prob.powi(index as i32) );
        Ok((current_success + self.state.constant).min(1.0f32))
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
