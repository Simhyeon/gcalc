use crate::{GcalcResult, GcalcError};
use crate::models::ProbType;

/// Calculate geometric series of given inputs
pub fn geometric_series(count: usize, probabilty: f32) -> f32 {
    let a = probabilty;
    let r = 1f32 - probabilty;
    let n = count as i32;

    // Formula
    a * (1f32 - r.powi(n) ) / (1f32 - r)
}

/// Calculate geometric_series with given qualficiation
pub fn geometric_series_qual(probabilty: f32, qalification: f32) -> usize {
    let t = qalification;
    let a = probabilty;
    let r = 1f32 - probabilty;
    let count_similar = (1f32 - (t * (1f32 - r) / a)).log(r);
    count_similar.ceil() as usize
}

/// Get probabilty as lenient as possible
pub fn get_prob_alap(number_str: &str, suffix: Option<&str>) -> GcalcResult<f32> {
    let mut number = number_str.to_owned();

    // Remove general suffix
    if number.ends_with("%") {
        number.pop();
    }

    // Remove custom suffix
    if let Some(suffix) = suffix {
        if number.ends_with(suffix) {
            number = number.trim_end_matches(suffix).to_owned();
        }
    }

    let number = number.parse::<f32>()?;

    get_number_as_fraction(number)
}

pub fn get_number_as_fraction(number: f32) -> GcalcResult<f32> {
    let prob: f32;
    if number >= 0.0f32 { 
        // CASE : 0.0 <= num <= 1.0
        if number <= 1.0f32 { 
            prob = number;
        } else if number <= 100.0f32 { // CASE : 1.0 < num <= 100.0
            prob = number / 100.0f32;
        } else {
            return Err(GcalcError::InvalidProb(format!("Probability \"{}\" is not a valid number", number)));
        }
        Ok(prob)
    } else {
        Err(GcalcError::InvalidProb(format!("Probability \"{}\" should be a positive number", number)))
    }
}

/// Get probabilty according to given type
///
/// Available types are
/// - Percentage
/// - Floating number
pub fn get_prob_as_formatted(mut num: f32, prob_type: &ProbType, precision: &Option<usize>) -> String {
    let mut num_string: String;

    // Add percentage
    if let ProbType::Percentage = prob_type { 
        num = num * 100f32; // multiply by 100
        num_string = float_to_string(num, precision);
        num_string.push_str("%");
    } else {
        num_string = float_to_string(num, precision);
    }

    num_string
}

/// Convert floating number to string 
///
/// This doesn't simply use a single format macro 
/// because format macro varies according to exponents of given number.
pub fn float_to_string(num: f32, precision: &Option<usize>) -> String {
    if let Some(precision) = precision {
        let decimal_precision = 10.0f32.powi(*precision as i32);
        let converted = f32::trunc(num  * decimal_precision ) / decimal_precision ;
        format!("{:.1$}",converted,precision)
    } else {
        num.to_string()
    }
}
