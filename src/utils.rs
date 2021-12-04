use crate::{calc::ProbType, GcalcResult, GcalcError};

/// Get probabilty according to given type
///
/// Available types are
/// - Percentage
/// - Floating number
pub fn get_prob_as_type(mut num: f32, prob_type: &ProbType, precision: &Option<usize>) -> String {
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

pub fn prob_sanity_check(num: f32) -> GcalcResult<()> {
    if num > 1.0f32 {
        return Err(GcalcError::InvalidProb(format!("Given number : {} is bigger than 1.0", num)));
    } else {
        Ok(())
    }
}
