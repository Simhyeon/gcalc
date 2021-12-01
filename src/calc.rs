use std::fs::File;

use crate::{GcalcResult, GcalcError};
use crate::formatter::Formatter;
use crate::utils;

pub struct Calculator {
    start_probability: f32,
    count: usize,
    prob_precision: Option<usize>,
    prob_type: ProbType,
}

// TODO 
// INclude this in calculator
pub enum Destination {
    File(File),
    Console,
}

pub enum ProbType {
    Percentage,
    Float,
}

// TODO
// Use minhash
// for bonus probabilty chart
// e.g.)
//
// 5th fail | 10%+
// 6th fail | 10%+
// 7th fail | 12%+
// 8th fail | 12%+
//
// Should be made of 
// ```rust
// ThinHash<usize,f32> 
// ```

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
            prob_precision: None,
            prob_type: ProbType::Float,
        })
    }

    pub fn prob_type(mut self, prob_type: ProbType) -> Self {
        self.prob_type = prob_type; 
        self
    }

    pub fn precision(mut self, precision: usize) -> GcalcResult<Self> {
        self.prob_precision.replace(precision);
        Ok(self)
    }

    // </BUILDER>

    // <PROCESSING>
    pub fn print_range(&self, range: Option<(usize, usize)>) -> GcalcResult<()> {
        let (min,max) = range.unwrap_or((0,self.count));
        if max > self.count {
            return Err(GcalcError::InvalidArgument(format!("Given max index {} is bigger than total count : {}",max,self.count)));
        }

        let mut records : Vec<Vec<String>> = Vec::new();
        let fail_prob = 1f32 - self.start_probability;
        let mut last_sum = 0f32;
        for index in 0..self.count {
            self.gsec_single(fail_prob, &mut records, &mut last_sum, index);
        }

        // self.print_csv(vec!["count", "probabilty"],records, (min,max))?;
        self.print_table(vec!["count", "probabilty"],records, (min,max))?;
        Ok(())
    }

    pub fn print_until(&self, target_probability: f32) -> GcalcResult<()> {
        if target_probability >= 1.0f32 {
            return Err(GcalcError::InvalidArgument(format!("Given probabilty {} is bigger than 1.0", target_probability)));
        }

        let mut records : Vec<Vec<String>> = Vec::new();
        let fail_prob = 1f32 - self.start_probability;
        let mut last_sum = 0f32;
        let mut index = 0;
        while last_sum < target_probability {
            self.gsec_single(fail_prob, &mut records, &mut last_sum, index);
            index = index + 1;
        }

        // Caculate once more
        // NOTE
        // Don't have to add index becuase index is already added at the final while loop
        self.gsec_single(fail_prob, &mut records, &mut last_sum, index);

        // self.print_csv(vec!["count", "probabilty"],records, (0,index))?;
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
            self.gsec_single(fail_prob, &mut records, &mut last_sum, index);
            index = index + 1;
        }
        let total_cost = utils::float_to_string(index as f32 * cost.unwrap_or(0f32), &self.prob_precision);
        let values = vec![vec![index.to_string(), total_cost]];

        // self.print_csv(vec!["count", "probabilty"],records, (0,index))?;
        self.print_table(vec!["count", "cost"],values, (0,index))?;

        Ok(())
    }

    // </PROCESSING>


    // <INTERNAL>
    /// Geometric sequence single take
    fn gsec_single(&self,fail_prob: f32, records: &mut Vec<Vec<String>>, last_sum: &mut f32 ,index: usize) {
        let current_success = self.start_probability * ( fail_prob.powi(index as i32) );
        *last_sum = *last_sum + current_success;
        let prob = utils::get_prob_as_type(*last_sum, &self.prob_type, &self.prob_precision);
        records.push(vec![(index + 1).to_string(), prob]);
    }


    /// Intera method for print table to console
    fn print_csv(
        &self,
        headers: Vec<impl AsRef<str>>,
        mut values :Vec<Vec<String>>,
        range: (usize, usize)
    ) -> GcalcResult<()> {
        let headers = headers.into_iter().map(|s| s.as_ref().to_owned()).collect();
        values.insert(0, headers);

        match Formatter::to_csv_string(values, range) {
            Ok(csv) => println!("{}", csv),
            Err(err) => return Err(GcalcError::FormatFail(err)),
        }

        Ok(())
    }

    fn print_table(
        &self,
        headers: Vec<impl AsRef<str>>,
        mut values :Vec<Vec<String>>,
        range: (usize, usize)
    ) -> GcalcResult<()> {
        let headers = headers.into_iter().map(|s| s.as_ref().to_owned()).collect();
        values.insert(0, headers);
        let table = Formatter::to_table(values, range);
        table.printstd();
        Ok(())
    }
    // </INTERNAL>
}
