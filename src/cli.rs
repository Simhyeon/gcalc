use crate::{GcalcResult, Calculator, utils, TableFormat, ProbType};
use clap::{ArgMatches,App, Arg};

pub struct Cli;

impl Cli {
    pub fn run() -> GcalcResult<()> {
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
                .arg(Arg::new("PROB").required(true).about("Target probabilty").takes_value(true))
                .arg(Arg::new("reference").about("Reference file").short('r').long("ref").takes_value(true))
                .arg(Arg::new("count").required(true).about("Counts to execute").short('c').long("count").takes_value(true))
                .arg(Arg::new("start").about("Starting index to print").short('s').long("start").takes_value(true))
                .arg(Arg::new("format").about("Table format").short('f').long("format").takes_value(true))
                .arg(Arg::new("precision").about("Precision").short('p').long("precision").takes_value(true))
                .arg(Arg::new("probtype").about("Probabilty type").short('t').long("type").takes_value(true))
            ) // range subcommand
            .subcommand(App::new("reference"))
            .get_matches()
    }

    fn run_calculator(args: &ArgMatches) -> GcalcResult<()> {
        match args.subcommand() {
            Some(( "range" , range_m)) => {
                Self::subcommand_range(range_m)?;
            }
            Some(( "reference" , _)) => {
                Self::subcommand_reference()?;
            }
            _ => eprintln!("No proper sub command was given to the program"),
        }

        Ok(())
    }

    fn subcommand_range(args: &ArgMatches) -> GcalcResult<()> {
        let probabilty = args.value_of("PROB").unwrap().parse().expect("Probabilty should be float");
        utils::prob_sanity_check(probabilty)?;
        let count = args.value_of("count").unwrap().parse().expect("count should be integer");
        let mut cal = Calculator::new(probabilty,count)?;
        let mut min = 0;

        if let Some(csv_file) = args.value_of("reference") {
            cal.set_csv_file(std::path::Path::new(csv_file));
        }

        if let Some(start) = args.value_of("start") {
            min = start.parse().expect("Start should be integer");
        }

        if let Some(format) = args.value_of("format") {
            cal.set_table_format(TableFormat::from_str(format)?);
        }

        if let Some(precision) = args.value_of("precision") {
            cal.set_precision(precision.parse().expect("Failed to get precisino as usize"));
        }

        if let Some(prob_type) = args.value_of("probtype") {
            cal.set_prob_type(ProbType::from_str(prob_type)?);
        }

        cal.print_range(Some((min,count)))?;
        Ok(())
    }

    fn subcommand_reference() -> GcalcResult<()> {
        std::fs::write(std::path::Path::new("ref.csv"), r#"count,probabilty,bonus,cost"#)?;
        Ok(())
    }
}
