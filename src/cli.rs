use crate::GcalcResult;
use clap::{ArgMatches,App, Arg};

pub struct Cli;

impl Cli {
    pub fn run(&mut self) -> GcalcResult<()> {
        let cli_args = Cli::args_builder();
        Cli::run_calculator(&cli_args)?;
        Ok(())
    }

    fn args_builder() -> ArgMatches {
        App::new("gclac")
            .version("0.1.0")
            .author("Simon creek <simoncreek@tutanota.com>")
            .about("Gcalc is a gacha simulator for game development and other decision makings.") // meta information end
            .subcommand(
                App::new("range")
                .arg(Arg::new("PROB").required(true).about("Target probabilty"))
                .arg(Arg::new("count").required(true).about("Counts to execute"))
                .arg(Arg::new("start").about("Starting index to print"))
                .arg(Arg::new("").about("Table format"))
            ) // range subcommand
            .get_matches()
    }

    fn run_calculator(args: &ArgMatches) -> GcalcResult<()> {
        Ok(())
    }
}
