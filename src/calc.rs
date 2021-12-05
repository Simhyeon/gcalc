use std::collections::HashMap;
use std::path::{Path, PathBuf};
use csv::{Reader, StringRecordsIntoIter};

use crate::{GcalcResult, GcalcError};
use crate::formatter::Formatter;
use crate::utils;

// const COUNT_INDEX       : usize = 0;
const BASIC_PROB_INDEX  : usize = 1;
const BONUS_PROB_INDEX  : usize = 2;
const BASIC_COST_INDEX  : usize = 3;

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
    behaviour: CsvBehaviour,
}

enum CsvBehaviour {
    Repeat,
    Panic,
    // Possiblye early return
}

struct CalcState {
    pub probability: f32,
    pub constant: f32,
    pub cost: f32,
    pub success_until: f32,
    pub fail_until: f32,
}

impl CalcState {
    pub fn new(probabilty: f32) -> Self {
        Self {
            probability: probabilty,
            constant: 0.0,
            cost: 0.0,
            success_until: 0.0,
            fail_until: 1.0,
        }
    }
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
    pub fn new(start_probability: f32) -> GcalcResult<Self> {
        if start_probability > 1.0f32 {
            return Err(GcalcError::InvalidArgument(format!("Given probabilty of {} which should not be bigger than 1.0", start_probability)));
        }

        Ok(Self {
            state : CalcState::new(start_probability),
            count : 0,
            csv_file : None,
            header_map: None,
            format: TableFormat::Csv,
            prob_precision: None,
            target_probability: None,
            budget: None,
            prob_type: ProbType::Float,
            behaviour : CsvBehaviour::Repeat,
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

    pub fn cost(mut self, cost: f32) -> Self {
        self.state.cost = cost;
        self
    }

    // </BUILDER>
    
    // <Setter>
    pub fn set_cost(&mut self, cost: f32) {
        self.state.cost = cost;
    }

    pub fn set_target_probability(&mut self, target_probability: f32) -> GcalcResult<()> {
        if target_probability > 1.0f32 || target_probability < 0.0f32 {
            return Err(GcalcError::InvalidArgument(format!("Given probabilty \"{}\" is should be bigger than 0.0 and smaller than 1.0", target_probability)));
        }

        self.target_probability.replace(target_probability);
        Ok(())
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
    pub fn print_range(&mut self, count: usize,start_index: Option<usize>) -> GcalcResult<()> {
        self.count = count;
        let records = self.create_records(true)?;

        self.print_table(vec!["count", "probabilty", "cost"],records, Some((start_index.unwrap_or(0),self.count)))?;
        Ok(())
    }

    pub fn print_conditional(&mut self) -> GcalcResult<()> {
        let records = self.create_records(false)?;

        self.print_table(vec!["count", "probabilty", "cost"],records, None)?;
        Ok(())
    }

    pub fn print_qualfication(&mut self) -> GcalcResult<()> {
        let records = self.create_records(false)?;
        let total_count = records.len();
        println!("{:?}", records);
        let values = vec![
            vec![total_count.to_string(), records[total_count - 1][2].clone() ]
        ];

        self.print_table(vec!["count", "cost"],values, None)?;

        Ok(())
    }

    /// Creat recors accroding to miscellaenous states
    fn create_records(&mut self, use_range:bool) -> GcalcResult<Vec<Vec<String>>> {

        // !use_range means it is used as conditional loop
        // Thus, at least one condition should be given or say sanity check 
        if !use_range { self.conditional_sanity_check()?; }

        // Total csv records iterator
        let mut csv_records = if let Some(file) = &self.csv_file {
            Some(Reader::from_path(file).expect("Failed to read csv from given file").into_records())
        } else { None };

        let mut records : Vec<Vec<String>> = Vec::new();
        let mut total_cost = 0f32;
        let mut index = 0;

        loop {
            self.update_state_from_csv_file(&mut csv_records, index)?;
            self.update_state_prob()?;

            let prob_str = utils::get_prob_as_type(
                self.state.success_until,
                &self.prob_type,
                &self.prob_precision
            );
            records.push(vec![(index + 1).to_string(), prob_str, total_cost.to_string()]);
            total_cost = total_cost + self.state.cost;
            
            // If current probabilty is bigger than target_probability break
            if let Some(target) = self.target_probability {
                if self.state.success_until > target { break; }
            }

            // If current cost is bigger than budget, break;
            if let Some(budget) = self.budget {
                if total_cost > budget { break; }
            }
            index = index + 1;

            // When using range variant,
            // break when loop reached max count
            if use_range && index >= self.count { break; }
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

                    if row.len() > BONUS_PROB_INDEX {
                        self.state.constant = row[BONUS_PROB_INDEX]
                            .parse()
                            .expect("Failed to parse");
                    }
                    if row.len() > BASIC_PROB_INDEX {
                        self.state.probability = row[BASIC_PROB_INDEX]
                            .parse()
                            .expect("Failed to parse");
                    }
                    if row.len() > BASIC_COST_INDEX {
                        self.state.cost = row[BASIC_COST_INDEX]
                            .parse()
                            .expect("Failed to parse");
                    }
                }
                None => { 
                    match self.behaviour {
                        CsvBehaviour::Repeat => (), // Do nothing & respect previous value,
                        CsvBehaviour::Panic => {
                            return Err(GcalcError::CsvError(format!("Empty row in index: {}", index))); 
                        }
                    }
                }
            }
        } 
        Ok(())
    }

    /// Update state probability
    fn update_state_prob(&mut self) -> GcalcResult<()> {
        utils::prob_sanity_check(self.state.constant)?;
        // Current indenpendent success rate
        let success = (self.state.probability + self.state.constant).min(1.0f32);
        self.state.success_until = self.state.success_until + self.state.fail_until * success;
        let fail_prob = (1f32 - success).max(0.0f32);
        // Fail until is multiplied
        self.state.fail_until = self.state.fail_until * fail_prob;
        Ok(())
    }

    fn conditional_sanity_check(&self) -> GcalcResult<()> {
        // Both empty
        if self.target_probability == None && self.budget == None {
            return Err(GcalcError::InvalidConditional(format!("Either target probability or budget should be present")));
        }

        if self.csv_file == None && self.budget != None && self.state.cost == 0.0 {
            return Err(GcalcError::InvalidConditional(format!("0 cost with budget will incur infinite loop")));
        }

        if self.csv_file == None && self.target_probability != None && self.state.probability == 0.0 {
            return Err(GcalcError::InvalidConditional(format!("0 probability with target probability will incur infinite loop")));
        }

        Ok(())
    }

    fn print_table(
        &self,
        headers: Vec<impl AsRef<str>>,
        mut values :Vec<Vec<String>>,
        range: Option<(usize, usize)>
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
                Formatter::to_console_table(values, range).printstd();
            }
        }
        Ok(())
    }
    // </INTERNAL>
}
