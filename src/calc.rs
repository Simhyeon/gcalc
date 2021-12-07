use csv::StringRecordsIntoIter;

use crate::models::{Record, Qualficiation, ColumnMap, RefCsv};
use crate::{GcalcResult, GcalcError};
use crate::formatter::{RecordFormatter, QualFormatter};
use crate::utils;
use crate::consts::*;

// TODO 
// Csv file
pub struct Calculator {
    state: CalcState,
    count: usize,
    format: TableFormat,
    csv_ref: RefCsv,
    csv_no_header: bool,
    column_map: ColumnMap,
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
    pub fn new(probability: f32) -> Self {
        Self {
            probability,
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
    GFM,
}

impl TableFormat {
    pub fn from_str(string : &str) -> GcalcResult<Self> {
        match string.to_lowercase().as_str() {
            "console" => Ok(Self::Console),
            "csv" => Ok(Self::Csv),
            "gfm" | "github" => Ok(Self::GFM),
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
            return Err(GcalcError::InvalidArgument(format!("Given probability of {} which should not be bigger than 1.0", start_probability)));
        }

        Ok(Self {
            state : CalcState::new(start_probability),
            count : 0,
            csv_ref : RefCsv::None,
            csv_no_header: false,
            column_map: ColumnMap::new(COUNT_INDEX, BASIC_PROB_INDEX, ADDED_PROB_INDEX, COST_INDEX),
            format: TableFormat::Csv,
            prob_precision: None,
            target_probability: None,
            budget: None,
            prob_type: ProbType::Float,
            behaviour : CsvBehaviour::Repeat,
        })
    }

    pub fn no_header(mut self, tv: bool) -> Self {
        self.csv_no_header = tv;
        self
    }

    pub fn column_map(mut self, column_map: ColumnMap) -> Self {
        self.column_map = column_map;
        self
    }

    pub fn panic_on_invlaid_csv(mut self, tv: bool) -> Self {
        if tv { self.behaviour = CsvBehaviour::Panic }
        else { self.behaviour = CsvBehaviour::Repeat }
        self
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

    pub fn csv_ref(mut self, csv_reference: RefCsv) -> Self {
        self.csv_ref = csv_reference;
        self
    }

    pub fn cost(mut self, cost: f32) -> Self {
        self.state.cost = cost;
        self
    }

    // </BUILDER>
    
    // <Setter>
    
    pub fn set_column_map(&mut self, column_map: ColumnMap) {
        self.column_map = column_map;
    }

    pub fn set_no_header(&mut self, tv: bool) {
        self.csv_no_header = tv;
    }

    pub fn set_cost(&mut self, cost: f32) {
        self.state.cost = cost;
    }

    pub fn set_target_probability(&mut self, target_probability: f32) -> GcalcResult<()> {
        if target_probability > 1.0f32 || target_probability < 0.0f32 {
            return Err(GcalcError::InvalidArgument(format!("Given probability \"{}\" is should be bigger than 0.0 and smaller than 1.0", target_probability)));
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

    pub fn set_csv_file(&mut self, csv_reference: RefCsv) {
        self.csv_ref = csv_reference;
    }
    // </Setter>

    // <PROCESSING>
    pub fn print_range(&mut self, count: usize,start_index: Option<usize>) -> GcalcResult<()> {
        // Update count for calculation
        self.count = count;
        let records = self.create_records(true)?;
        self.print_records(records, Some((start_index.unwrap_or(0),self.count)))?;
        Ok(())
    }

    pub fn print_conditional(&mut self) -> GcalcResult<()> {
        let records = self.create_records(false)?;
        self.print_records(records, None)?;
        Ok(())
    }

    pub fn print_qualfication(&mut self) -> GcalcResult<()> {
        let records = self.create_records(false)?;

        let total_count = records.len();
        let total_cost = records.last().unwrap_or(&Record::new(0,"".to_string(), 0.0)).cost;

        self.print_qual_table(total_count, total_cost);

        Ok(())
    }

    /// Creat recors accroding to miscellaenous states
    fn create_records(&mut self, use_range:bool) -> GcalcResult<Vec<Record>> {

        // "!use_range" (negation of use range) means it is used as conditional loop
        // Thus, at least one condition should be given or say sanity check 
        if !use_range { self.conditional_sanity_check()?; }

        // Uniform format of csv value as string
        let csv_value = match &self.csv_ref {
            RefCsv::File(file) => std::fs::read_to_string(file)?,
            RefCsv::Raw(string) => string.clone(),
            RefCsv::None => "".to_owned()
        };

        // Total csv records iterator
        let mut csv_record = csv::ReaderBuilder::new()
            .has_headers(!self.csv_no_header)
            .from_reader(csv_value.as_bytes())
            .into_records();

        let mut records : Vec<Record> = Vec::new();
        let mut total_cost = 0f32;
        let mut index = 0;

        loop {
            // Only if csv value is not empty, update the state from csv value(file)
            if csv_value != "" { self.update_state_from_csv_file(&mut csv_record, index)?; }
            self.calculate_fail_success()?;

            let prob_str = utils::get_prob_as_formatted(
                self.state.success_until,
                &self.prob_type,
                &self.prob_precision
            );
            records.push(Record::new(index + 1,prob_str.to_owned(), total_cost));
            total_cost = total_cost + self.state.cost;
            
            // If current probability is bigger than target_probability break
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
    fn update_state_from_csv_file(&mut self, csv_records :&mut StringRecordsIntoIter<&[u8]>, index: usize) -> GcalcResult<()> {
        match csv_records.next() {
            Some(row) => {
                let row = row.expect("Failed to parse row"); // Temporary bound
                let row = row.iter().collect::<Vec<&str>>();

                if row.len() > self.column_map.probability {
                    self.state.probability = row[self.column_map.probability]
                        .parse()
                        .expect("Failed to parse");
                }

                if row.len() > self.column_map.added {
                    self.state.constant = row[self.column_map.added]
                        .parse()
                        .expect("Failed to parse");
                }

                if row.len() > self.column_map.cost {
                    self.state.cost = row[self.column_map.cost]
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
        Ok(())
    }

    /// calculate fail success
    fn calculate_fail_success(&mut self) -> GcalcResult<()> {
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

        if self.csv_ref == RefCsv::None && self.budget != None && self.state.cost == 0.0 {
            return Err(GcalcError::InvalidConditional(format!("0 cost with budget will incur infinite loop")));
        }

        if self.csv_ref == RefCsv::None && self.target_probability != None && self.state.probability == 0.0 {
            return Err(GcalcError::InvalidConditional(format!("0 probability with target probability will incur infinite loop")));
        }

        Ok(())
    }

    fn print_records(
        &self,
        records :Vec<Record>,
        range: Option<(usize, usize)>
    ) -> GcalcResult<()> {
        let formatted = match self.format {
            TableFormat::Csv => {
                match RecordFormatter::to_raw_csv(records, range) {
                    Ok(csv) => csv,
                    Err(err) => return Err(GcalcError::FormatFail(err)),
                }
            }
            TableFormat::Console => {
                RecordFormatter::to_styled_table(records, range, tabled::Style::default())
            }
            TableFormat::GFM => {
                RecordFormatter::to_styled_table(records, range, tabled::Style::github_markdown())
            }
        };
        print!("{}", formatted);
        Ok(())
    }

    fn print_qual_table(&self, count: usize, cost: f32) {
        let formatted = match self.format {
            TableFormat::Csv => {
                QualFormatter::to_csv_table(Qualficiation::new(count, cost)).expect("Failed to get csv table from qualification")
            }
            TableFormat::Console => {
                QualFormatter::to_styled_table(Qualficiation::new(count, cost), tabled::Style::default())
            }
            TableFormat::GFM => {
                QualFormatter::to_styled_table(Qualficiation::new(count, cost), tabled::Style::github_markdown())
            }
        };
        print!("{}", &formatted);

    }
    // </INTERNAL>
}
