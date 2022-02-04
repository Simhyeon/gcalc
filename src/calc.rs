use std::path::Path;

use csv::StringRecordsIntoIter;

use crate::models::{Record, Qualficiation, ColumnMap, CsvRef, OutOption, RecordCursor, CSVInvalidBehaviour, ProbType};
#[cfg(feature = "plotters")]
use crate::plot::{PlotAttribute, Renderer};
use crate::{GcalcResult, GcalcError};
use crate::formatter::{RecordFormatter, QualFormatter};
use crate::utils;
use crate::consts::*;
#[cfg(feature = "tabled")]
use tabled;
#[cfg(feature = "option")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "option")]
#[derive(Serialize, Deserialize)]
pub struct CalculatorOption {
    count: usize,
    prob_type: ProbType,
    prob_precision : Option<usize>,
    budget: Option<f32>,
    fallback: CSVInvalidBehaviour,
    no_header: bool,
    strict: bool,
    target: Option<f32>,
    column_map: ColumnMap,
    // Non-wasm exclusive options
    format: TableFormat,
    csv_ref: CsvRef, // -> For wasm it should be defined differently
    out_option : OutOption,
}

#[cfg(feature = "option")]
impl CalculatorOption {
    pub fn new() -> Self {
        Self {
            count: 0,
            prob_type: ProbType::Fraction,
            prob_precision : None,
            budget: None,
            fallback: CSVInvalidBehaviour::None,
            no_header: false,
            strict: false,
            target: None,
            column_map: ColumnMap::default(),
            // Non-wasm exclusive options
            format: TableFormat::CSV,
            csv_ref: CsvRef::None, // -> For wasm it should be defined differently
            out_option : OutOption::Console,
        }
    }

    pub fn to_json(&self) -> GcalcResult<String> {
        Ok(serde_json::to_string_pretty(self).expect("Failed to serialize option to json"))
    }

    pub fn from_file(path: &std::path::Path) -> GcalcResult<Self> {
        let option = serde_json::from_str(&std::fs::read_to_string(path)?).expect("Failed to read option file");
        Ok(option)
    }
}

// TODO 
// Csv file
pub struct Calculator {
    state: CalcState,
    count: usize,
    offset: Option<usize>,
    format: TableFormat,
    csv_ref: CsvRef,
    csv_no_header: bool,
    column_map: ColumnMap,
    csv_invalid_behaviour: CSVInvalidBehaviour,
    prob_precision: Option<usize>,
    budget: Option<f32>,
    target_probability : Option<f32>,
    prob_type: ProbType,
    // Which behaviour to take when csv rows ends
    record_behaviour: CsvRecordBehaviour, // Strict option
    out_option: OutOption,
}
impl Calculator {
    // <BUILDER>
    // Constructor methods
    pub fn new() -> GcalcResult<Self> {
        Ok(Self {
            state : CalcState::new(),
            count : 0,
            offset: None,
            csv_ref : CsvRef::None,
            csv_no_header: false,
            column_map: ColumnMap::new(COUNT_INDEX, BASIC_PROB_INDEX, ADDED_PROB_INDEX, COST_INDEX),
            format: TableFormat::CSV,
            csv_invalid_behaviour: CSVInvalidBehaviour::None,
            prob_precision: None,
            target_probability: None,
            budget: None,
            prob_type: ProbType::Fraction,
            record_behaviour : CsvRecordBehaviour::Repeat,
            out_option: OutOption::Console,
        })
    }

    #[cfg(feature = "option")]
    pub fn option(mut self, option: &CalculatorOption) -> Self {
        self.count = option.count;
        self.prob_type = option.prob_type;
        self.prob_precision  = option.prob_precision;
        self.budget = option.budget;
        self.csv_invalid_behaviour = option.fallback;
        self.csv_no_header = option.no_header;
        self.set_strict_csv(option.strict);
        self.target_probability = option.target;
        self.column_map = option.column_map;
        self.format = option.format;
        self.csv_ref = option.csv_ref.clone();
        self.out_option = option.out_option.clone();
        self
    }

    pub fn no_header(mut self, tv: bool) -> Self {
        self.csv_no_header = tv;
        self
    }

    pub fn column_map(mut self, column_map: ColumnMap) -> Self {
        self.column_map = column_map;
        self
    }

    pub fn strict_csv(mut self, tv: bool) -> Self {
        if tv { self.record_behaviour = CsvRecordBehaviour::Panic }
        else { self.record_behaviour = CsvRecordBehaviour::Repeat }
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

    pub fn probability(mut self, probability: f32) -> GcalcResult<Self> {
        let probability = utils::get_number_as_fraction(probability)?;
        self.state.probability = probability;
        self.state.initial_probability = probability;
        Ok(self)
    }

    pub fn constant(mut self, constant: f32) -> GcalcResult<Self> {
        let constant = utils::get_number_as_fraction(constant)?;
        self.state.constant = constant;
        self.state.initial_constant = constant;
        Ok(self)
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
        self.state.initial_cost = cost;
        self
    }

    pub fn out_file(mut self, path: &Path) -> Self {
        self.out_option= OutOption::File(path.to_owned());
        self
    }

    pub fn csv_fallback(mut self, behaviour: &str) -> GcalcResult<Self> {
        self.csv_invalid_behaviour= CSVInvalidBehaviour::from_str(behaviour)?;
        Ok(self)
    }
    // </BUILDER>
    
    // <SETTER>
    #[cfg(feature = "option")]
    pub fn set_option(&mut self, option: &CalculatorOption) {
        let option = option.to_owned();
        self.count = option.count;
        self.prob_type = option.prob_type;
        self.prob_precision  = option.prob_precision;
        self.budget = option.budget;
        self.csv_invalid_behaviour = option.fallback;
        self.csv_no_header = option.no_header;
        self.set_strict_csv(option.strict);
        self.target_probability = option.target;
        self.column_map = option.column_map;
        self.format = option.format;
        self.csv_ref = option.csv_ref.clone();
        self.out_option = option.out_option.clone();
    }

    pub fn set_column_map(&mut self, column_map: ColumnMap) {
        self.column_map = column_map;
    }

    pub fn set_no_header(&mut self, tv: bool) {
        self.csv_no_header = tv;
    }

    pub fn set_probability(&mut self, probability: f32, update_initial_value: bool) -> GcalcResult<()>  {
        let probability = utils::get_number_as_fraction(probability)?;
        self.state.probability = probability;
        if update_initial_value { self.state.initial_probability = probability; }
        Ok(())
    }

    pub fn set_cost(&mut self, cost: f32, update_initial_value: bool) {
        self.state.cost = cost;
        if update_initial_value { self.state.initial_cost = cost; }
    }

    pub fn set_constant(&mut self, constant: f32, update_initial_value: bool) -> GcalcResult<()> {
        let constant = utils::get_number_as_fraction(constant)?;
        self.state.constant = constant;
        if update_initial_value { self.state.initial_constant = constant; }
        Ok(())
    }

    pub fn set_strict_csv(&mut self, tv: bool) {
        if tv { self.record_behaviour = CsvRecordBehaviour::Panic }
        else { self.record_behaviour = CsvRecordBehaviour::Repeat }
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

    pub fn set_offset(&mut self, offset: usize) {
        self.offset.replace(offset);
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

    pub fn set_csv_value_fallback(&mut self, behaviour: &str) -> GcalcResult<()> {
        self.csv_invalid_behaviour= CSVInvalidBehaviour::from_str(behaviour)?;
        Ok(())
    }
    // </SETTER>

    // <PROCESSING>
    pub fn print_range(
        &mut self,
        count: Option<usize>,
        start_index: Option<usize>,
        #[cfg(feature = "plotters")]
        plot: bool
    ) -> GcalcResult<()> {
        // Update count for calculation
        if let Some(count) = count {
            self.count = count;
        }
        let records = self.create_records(true)?;
        self.print_records(&records, Some((start_index.unwrap_or(0),self.count)))?;
        #[cfg(feature = "plotters")]
        if plot {
            Renderer::draw_chart(PlotAttribute::default(), &records, &self.prob_type)?;
        }
        Ok(())
    }

    pub fn print_conditional(
        &mut self,
        #[cfg(feature = "plotters")]
        plot: bool
    ) -> GcalcResult<()> {
        let records = self.create_records(false)?;
        self.print_records(&records, None)?;
        #[cfg(feature = "plotters")]
        if plot {
            Renderer::draw_chart(PlotAttribute::default(), &records, &self.prob_type)?;
        }
        Ok(())
    }

    pub fn print_qualfication(&mut self) -> GcalcResult<()> {
        self.conditional_sanity_check()?;
        let total_count: usize;
        let total_cost: f32;
        let final_probability: String;

        // Simply calculate geometric series
        if let CsvRef::None = self.csv_ref {

            if self.state.probability >= 1.0 {
                total_count = 1;
                total_cost = self.state.cost;
                final_probability = utils::get_prob_as_formatted(1.0f32, &self.prob_type, &self.prob_precision);
            } 
            // Probability and possibly with budget
            else if let Some(target) = self.target_probability {
                let count = utils::geometric_series_qual(self.state.probability + self.state.constant, target);
                let count = if let Some(bud) = self.budget  {
                    if count as f32 * self.state.cost > bud {
                        (bud / self.state.cost).floor() as usize
                    } else { count }
                } else { count };
                total_count = count;
                total_cost = count as f32 * self.state.cost; 
                final_probability = utils::get_prob_as_formatted(
                    utils::geometric_series(total_count, self.state.probability + self.state.constant), 
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
                    utils::geometric_series(total_count, self.state.probability + self.state.constant), 
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

        // Add more records if offset is given
        if let Some(mut offset) = self.offset {
            // This is a non dry code from previous loop
            record_index += 1;
            while offset > 0 {
                cursor = RecordCursor::Next;

                // Only if csv value is not empty, update the state from csv value(file)
                if !csv_value.is_empty() { self.update_state_from_csv_file(&mut csv_record, csv_index, &mut cursor)?; }
                self.calculate_fail_success()?;

                let prob_str = utils::get_prob_as_formatted(
                    self.state.success_until,
                    &self.prob_type,
                    &self.prob_precision
                );

                total_cost = total_cost + self.state.cost;
                records.push(Record::new(record_index + 1,prob_str.to_owned(), total_cost));

                // If and only if cursor is next,
                // increase csv index
                if let RecordCursor::Next = cursor {
                    csv_index += 1;
                }

                // Increases record index 
                // regardless of csv_index
                record_index += 1;

                if offset == 0 { break } else { offset -= 1; }
            }
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
                        Err(err)  => {
                            match self.csv_invalid_behaviour { 
                                CSVInvalidBehaviour::None => return Err(err),                                  // this is error
                                CSVInvalidBehaviour::Ignore => (),                                             // Do not update value
                                CSVInvalidBehaviour::Rollback => self.state.probability = self.state.initial_probability,   // Use default value
                            }
                        }
                    }
                }

                // Get constant probability
                if row.len() > self.column_map.constant {
                    match utils::get_prob_alap(row[self.column_map.constant],None) {
                        Ok(value) => self.state.constant = value,
                        Err(err)  => {
                            match self.csv_invalid_behaviour { 
                                CSVInvalidBehaviour::None => return Err(err),                                  // this is error
                                CSVInvalidBehaviour::Ignore => (),                                             // Do not update value
                                CSVInvalidBehaviour::Rollback => self.state.constant = self.state.initial_constant,      // Use default value
                            }
                        }
                    }
                }

                // Get cost
                if row.len() > self.column_map.cost {
                    match row[self.column_map.cost].parse::<f32>() {
                        Ok(value) => self.state.cost = value,
                        Err(_) => {
                            match self.csv_invalid_behaviour { 
                                CSVInvalidBehaviour::None => {                                                     // this is error
                                    return Err(GcalcError::ParseError(
                                            format!(
                                                "Cost should be a number, but the value in ({},{}) is not", 
                                                index + 1, 
                                                self.column_map.cost
                                            )))
                                },                                                     
                                CSVInvalidBehaviour::Ignore => (),                                                 // Do not update value
                                CSVInvalidBehaviour::Rollback => self.state.cost = self.state.initial_cost,      // Use default value
                            }
                        }
                    }

                }
            } // End some match
            None => { // Record not found 
                match self.record_behaviour {
                    CsvRecordBehaviour::Repeat => (), // Do nothing & respect previous value,
                    CsvRecordBehaviour::Panic => {
                        return Err(GcalcError::CsvError(format!("Empty row in index: {}", index + 1))); 
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
        records :&Vec<Record>,
        range: Option<(usize, usize)>
    ) -> GcalcResult<()> {
        let formatted = match self.format {
            TableFormat::CSV => {
                match RecordFormatter::to_raw_csv(records, range) {
                    Ok(csv) => csv,
                    Err(err) => return Err(GcalcError::FormatFail(err)),
                }
            }
            #[cfg(feature = "tabled")]
            TableFormat::Console => {
                RecordFormatter::to_styled_table(records, range, tabled::Style::default())
            }
            #[cfg(feature = "tabled")]
            TableFormat::GFM => {
                RecordFormatter::to_styled_table(records, range, tabled::Style::github_markdown())
            }
        };
        self.yield_table(&formatted)?;
        Ok(())
    }

    fn print_qual_table(&self, count: usize, cost: f32, probability: &str) -> GcalcResult<()> {
        let formatted = match self.format {
            TableFormat::CSV => {
                QualFormatter::to_csv_table(Qualficiation::new(count, cost, probability))?
            }
            #[cfg(feature = "tabled")]
            TableFormat::Console => {
                QualFormatter::to_styled_table(Qualficiation::new(count, cost, probability), tabled::Style::default())
            }
            #[cfg(feature = "tabled")]
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

enum CsvRecordBehaviour {
    Repeat,
    Panic,
    // Possibly early return
}

struct CalcState {
    pub probability: f32,
    pub initial_probability: f32,
    pub constant: f32,
    pub initial_constant: f32,
    pub cost: f32,
    pub initial_cost: f32,
    pub success_until: f32,
    pub fail_until: f32,
}

impl CalcState {
    pub fn new() -> Self {
        Self {
            probability : 1.0,
            initial_probability: 1.0,
            constant: 0.0,
            initial_constant: 0.0,
            cost: 0.0,
            initial_cost: 0.0,
            success_until: 0.0,
            fail_until: 1.0,
        }
    }
}

#[cfg_attr(feature= "option" ,derive(Serialize, Deserialize))]
#[derive(Clone,Copy)]
pub enum TableFormat {
    CSV,
    #[cfg(feature = "tabled")]
    GFM,
    #[cfg(feature = "tabled")]
    Console,
}

impl TableFormat {
    pub fn from_str(string : &str) -> GcalcResult<Self> {
        match string.to_lowercase().as_str() {
            #[cfg(feature = "tabled")]
            "console" => Ok(Self::Console),
            #[cfg(feature = "tabled")]
            "gfm" | "github" => Ok(Self::GFM),
            "csv" => Ok(Self::CSV),
            _ => Err(GcalcError::InvalidConversion(format!("{} is not a valid table format", string))),
        }
    }
}
