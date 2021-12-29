use crate::{calc::ProbType, GcalcResult, GcalcError};

/// Get probabilty as lenient as possible
pub fn get_prob_alap(number_str: &str, suffix: Option<&str>) -> GcalcResult<f32> {
    let prob : f32;
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

    let number = number.parse::<f32>().expect("Failed to parse number as f32");

    if number > 0.0f32 { 
        // CASE : 0.0 <= num <= 1.0
        if number <= 1.0f32 { 
            prob = number;
        } else if number <= 100.0f32 { // CASE : 1.0 < num <= 100.0
            prob = number / 100.0f32;
        } else {
            return Err(GcalcError::InvalidProb(format!("Probability {} is not a valid format", number_str)));
        }
        Ok(prob)
    } else {
        Err(GcalcError::InvalidProb(format!("Probability {} is not a valid format", number_str)))
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
/// This gets optional precision as formatting modifier
pub fn float_to_string(num: f32, precision: &Option<usize>) -> String {
    if let Some(precision) = precision {
        format!("{:.1$}",num,precision)
    } else {
        format!("{}",num)
    }
}
