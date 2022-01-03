use gcalc::GcalcResult;
#[cfg(feature = "binary")]
use gcalc::cli::Cli;

fn main() -> GcalcResult<()> {
    #[cfg(feature = "binary")]

    if let Err(err) = Cli::run() {
        // Propagate error to stdout
        eprintln!("{}", err);
    }
    Ok(())
}
