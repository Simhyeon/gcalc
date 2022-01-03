use wasm_bindgen::prelude::*;
use crate::{calc::{CalculatorOption, Calculator}, GcalcError};

// JS methods
#[wasm_bindgen]
extern "C" {
    //console.error
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(l : &str);
    //console.error
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn log_error(e: &JsValue);
}

type WasmResult<T> = Result<T, JsValue>;

impl From<GcalcError> for JsValue {
    fn from(err : GcalcError) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

/// Create default calculator option
#[wasm_bindgen]
pub fn default_option() -> CalculatorOption {
    CalculatorOption::new()
}

/// Calculate probabilities
#[wasm_bindgen]
pub fn calculate(command: &str, option: &CalculatorOption) -> JsValue {
    let calculation_result = match command {
        "range" => range(option),
        "cond" => cond(option),
        "qual" => qual(option),
        _ => {
            log_error(&JsValue::from_str(&format!("\"{}\" is not a valid command", command)));
            Err(JsValue::from(GcalcError::InvalidArgument(format!("\"{}\" is not a valid command", command))))
        }
    };
    match calculation_result {
        Err(js_error) => {
            log_error(&js_error);
            JsValue::from_str("")
        },
        Ok(csv) => { JsValue::from_str(&csv) }
    } 
}

#[wasm_bindgen]
pub fn range(option: &CalculatorOption) -> WasmResult<String> {
    let mut cal = Calculator::new()?;
    cal.set_option(option)?;
    cal.print_range(None,None)?;
    Ok(cal.calculated_csv)
}

#[wasm_bindgen]
pub fn cond(option: &CalculatorOption) -> WasmResult<String> {
    let mut cal = Calculator::new()?;
    cal.set_option(option)?;
    cal.print_conditional()?;
    Ok(cal.calculated_csv)
}

#[wasm_bindgen]
pub fn qual(option: &CalculatorOption) -> WasmResult<String>  {
    let mut cal = Calculator::new()?;
    cal.set_option(option)?;
    cal.print_qualfication()?;
    Ok(cal.calculated_csv)
}
