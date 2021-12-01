use crate::calc::ProbType;

/// Get probabilty according to given type
///
/// Available types are
/// - Percentage
/// - Floating number
pub fn get_prob_as_type(num: f32, prob_type: &ProbType, precision: &Option<usize>) -> String {
    let mut num = float_to_string(num, precision);

    // Add percentage
    if let ProbType::Percentage = prob_type { 
        num.push_str("%");
    }
    num
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
