#[cfg(feature = "binary")]
use gcalc::cli::Cli;
use gcalc::GcalcResult;

fn main() -> GcalcResult<()> {
    #[cfg(feature = "binary")]
    if let Err(err) = Cli::run() {
        use std::io::Write;
        // Propagate error to stdout
        writeln!(std::io::stderr(), "{}", err)?;
    }
    Ok(())
}
