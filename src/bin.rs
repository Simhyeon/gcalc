use gcalc::GcalcResult;
#[cfg(feature = "binary")]
use gcalc::cli::Cli;

// Usage
// gcalc -F test.csv -R 0,10 -B budget_number -U until_probabilty
// Maybe I should take single entry?

fn main() -> GcalcResult<()> {
    #[cfg(feature = "binary")]

    if let Err(err) = Cli::run() {
        // Propagate error to stdout
        eprintln!("{}", err);
    }
    Ok(())
}
