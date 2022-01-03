use std::path::Path;

use csv::StringRecordsIntoIter;

use crate::models::{Record, Qualficiation, ColumnMap, CsvRef, OutOption, RecordCursor};
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
    csv_ref: CsvRef,
    csv_no_header: bool,
    column_map: ColumnMap,
    fallable_csv: bool,
    prob_precision: Option<usize>,
    budget: Option<f32>,
    target_probability : Option<f32>,
    prob_type: ProbType,
    // Which behaviour to take when csv rows ends
    behaviour: CsvBehaviour,
    out_option: OutOption,
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
            csv_ref : CsvRef::None,
            csv_no_header: false,
            column_map: ColumnMap::new(COUNT_INDEX, BASIC_PROB_INDEX, ADDED_PROB_INDEX, COST_INDEX),
            format: TableFormat::Csv,
            fallable_csv: false,
            prob_precision: None,
            target_probability: None,
            budget: None,
            prob_type: ProbType::Float,
            behaviour : CsvBehaviour::Repeat,
            out_option: OutOption::Console,
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

    pub fn csv_ref(mut self, csv_reference: CsvRef) -> Self {
        self.csv_ref = csv_reference;
        self
    }

    pub fn cost(mut self, cost: f32) -> Self {
        self.state.cost = cost;
        self
    }

    pub fn out_file(mut self, path: &Path) -> Self {
        self.out_option= OutOption::File(path.to_owned());
        self
    }

    pub fn csv_fallable(mut self, tv: bool) -> Self {
        self.fallable_csv= tv;
        self
    }
    // </BUILDER>
    
    // <SETTER>
    pub fn set_column_map(&mut self, column_map: ColumnMap) {
        self.column_map = column_map;
    }

    pub fn set_no_header(&mut self, tv: bool) {
        self.csv_no_header = tv;
    }

    pub fn set_cost(&mut self, cost: f32) {
        self.state.cost = cost;
    }

    pub fn set_panic_on_invlaid_csv(&mut self, tv: bool) {
        if tv { self.behaviour = CsvBehaviour::Panic }
        else { self.behaviour = CsvBehaviour::Repeat }
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

    pub fn set_csv_file(&mut self, csv_reference: CsvRef) {
        self.csv_ref = csv_reference;
    }

    pub fn set_out_file(&mut self, path: &Path) {
        self.out_option = OutOption::File(path.to_owned());
    }

    pub fn set_csv_fallable(&mut self, tv: bool) {
        self.fallable_csv= tv;
    }
    // </SETTER>

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
        self.conditional_sanity_check()?;
        let total_count: usize;
        let total_cost: f32;
        let final_probability: String;

        // Simply calculate geometric series
        if let CsvRef::None = self.csv_ref {
            // Probability and possibly with budget
            if let Some(target) = self.target_probability {
                let count = utils::geometric_series_qual(self.state.probability, target);
                let count = if let Some(bud) = self.budget  {
                    if count as f32 * self.state.cost > bud {
                        (bud / self.state.cost).floor() as usize
                    } else { count }
                } else { count };
                total_count = count;
                total_cost = count as f32 * self.state.cost; 
                final_probability = utils::get_prob_as_formatted(
                    utils::geometric_series(total_count, self.state.probability), 
                    &self.prob_type, 
                    &self.prob_precision
                );
            } else { // No probability only budget
                if self.state.cost == 0f32 {
                    return Err(GcalcError::InvalidArgument(format!("Cost should not be 0 if no reference was given as argument.")));
                }
                let count = (self.budget.unwrap() / self.state.cost).floor() as usize;

                total_count = count;
                total_cost = count as f32 * self.state.cost;
                final_probability = utils::get_prob_as_formatted(
                    utils::geometric_series(total_count, self.state.probability), 
                    &self.prob_type, 
                    &self.prob_precision
                );
            }
        } else {
            let records = self.create_records(false)?;

            total_count = records.len();

            // It is a single record table so it is safe to index 0th element
            let last_record = &records[0];
            total_cost = last_record.cost;
            final_probability = last_record.probability.clone();
        }

        self.print_qual_table(total_count, total_cost, &final_probability)?;

        Ok(())
    }

    /// Creat records accroding to miscellaenous states
    fn create_records(&mut self, use_range:bool) -> GcalcResult<Vec<Record>> {

        // "!use_range" (negation of use range) means it is used as conditional loop
        // Thus, at least one condition should be given or say sanity check 
        if !use_range { self.conditional_sanity_check()?; }

        // Uniform format of csv value as string
        let csv_value = match &self.csv_ref {
            CsvRef::File(file) => std::fs::read_to_string(file)?,
            CsvRef::Raw(string) => string.clone(),
            CsvRef::None => "".to_owned()
        };

        // Total csv records iterator
        let mut csv_record = csv::ReaderBuilder::new()
            .has_headers(!self.csv_no_header)
            .from_reader(csv_value.as_bytes())
            .into_records();

        let mut records : Vec<Record> = Vec::new();
        let mut total_cost = 0f32;
        let mut record_index = 0;
        let mut csv_index = 0;
        let mut cursor : RecordCursor;

        loop {
            // Default cursor behaviour is next
            cursor = RecordCursor::Next;

            // Only if csv value is not empty, update the state from csv value(file)
            if !csv_value.is_empty() { self.update_state_from_csv_file(&mut csv_record, csv_index, &mut cursor)?; }
            self.calculate_fail_success()?;

            let prob_str = utils::get_prob_as_formatted(
                self.state.success_until,
                &self.prob_type,
                &self.prob_precision
            );
            // Because first try also consumes cost
            // total_cost should be calculated before push
            total_cost = total_cost + self.state.cost;
            records.push(Record::new(record_index + 1,prob_str.to_owned(), total_cost));
            
            // If current probability is bigger than target_probability break
            if let Some(target) = self.target_probability {
                if self.state.success_until > target { break; }
            }

            // If current cost is bigger than budget, break;
            if let Some(budget) = self.budget {
                if total_cost > budget { break; }
            }

            // If and only if cursor is next,
            // increase csv index
            if let RecordCursor::Next = cursor {
                csv_index += 1;
            }

            // Increases record index 
            // regardless of csv_index
            record_index += 1;

            // When using range variant,
            // break when loop reached max count
            if use_range && record_index >= self.count { break; }
        }

        Ok(records)
    }
    // </PROCESSING>

    // <INTERNAL>
    fn update_state_from_csv_file(&mut self, csv_records :&mut StringRecordsIntoIter<&[u8]>, index: usize, cursor: &mut RecordCursor) -> GcalcResult<()> {
        match csv_records.next() {
            Some(row) => {
                // Temporary bound
                let row = row?;
                let row = row.iter().collect::<Vec<&str>>();

                // Get Count,
                // and check if count is same with current index( which is acutally "index + 1" )
                if row.len() > self.column_map.count {
                    if row[self.column_map.count].parse::<usize>().expect("Failed to get count as number") > index + 1 {
                        *cursor = RecordCursor::Stay;
                        // Do nothing & respect previous value,
                        return Ok(());
                    }
                }

                // Get probability
                if row.len() > self.column_map.probability {
                    match utils::get_prob_alap(row[self.column_map.probability],None) {
                        Ok(value) => self.state.probability = value,
                        Err(err)  => if !self.fallable_csv { return Err(err); }
                    }
                }

                // Get added(bonus) probability
                if row.len() > self.column_map.added {
                    match utils::get_prob_alap(row[self.column_map.added],None) {
                        Ok(value) => self.state.constant = value,
                        Err(err)  => if !self.fallable_csv { return Err(err); }
                    }
                }

                // Get cost
                if row.len() > self.column_map.cost {
                    match row[self.column_map.cost].parse::<f32>() {
                        Ok(value) => self.state.cost = value,
                        Err(_)  => if !self.fallable_csv { 
                            return Err(GcalcError::ParseError(format!("Cost should be a number, but the value in ({},{}) is not", index + 1, self.column_map.cost)));
                        }
                    }

                }
            } // End some match
            None => { // Record not found 
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

        if self.csv_ref == CsvRef::None { // No ref file
            if self.budget != None && self.state.cost == 0.0 {
                return Err(GcalcError::InvalidConditional(format!("0 cost with budget will incur infinite loop")));
            }
            if self.target_probability != None && self.state.probability == 0.0 {
                return Err(GcalcError::InvalidConditional(format!("0 probability with static target probability will incur infinite loop")));
            }
            if let Some(num) = self.target_probability {
                if num == 1.0f32 && self.state.constant < 1.0f32 {
                    return Err(GcalcError::InvalidConditional(format!("1.0 probability cannot be reached. Use reference file if you need tailored control over probability.")));
                }
            }
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
        self.yield_table(&formatted)?;
        Ok(())
    }

    fn print_qual_table(&self, count: usize, cost: f32, probability: &str) -> GcalcResult<()> {
        let formatted = match self.format {
            TableFormat::Csv => {
                QualFormatter::to_csv_table(Qualficiation::new(count, cost, probability))?
            }
            TableFormat::Console => {
                QualFormatter::to_styled_table(Qualficiation::new(count, cost, probability), tabled::Style::default())
            }
            TableFormat::GFM => {
                QualFormatter::to_styled_table(Qualficiation::new(count, cost, probability), tabled::Style::github_markdown())
            }
        };
        self.yield_table(&formatted)?;
        Ok(())
    }

    fn yield_table(&self, table: &str) -> GcalcResult<()> {
        match &self.out_option {
            OutOption::Console => print!("{}", table),
            OutOption::File(path) => {
                if let Err(err) = std::fs::write(path, table.as_bytes()) {
                    eprintln!("File \"{}\" cannot be used as output redirection.", path.display());
                    return Err(GcalcError::StdIo(err));
                }
            }
        }
        Ok(())
    }
    // </INTERNAL>
}

enum CsvBehaviour {
    Repeat,
    Panic,
    // Possibly early return
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

