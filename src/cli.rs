use crate::{GcalcResult, Calculator, utils, TableFormat, ProbType, models::{RefCsv, ColumnMap}, GcalcError};
use clap::{ArgMatches,App, Arg};
use std::{path::PathBuf, io::Read};
use crate::consts::*;

pub struct Cli;

impl Cli {
    pub fn run() -> GcalcResult<()> {
        let cli_args = Cli::args_builder();
        Cli::run_calculator(&cli_args)?;
        Ok(())
    }

    fn args_builder() -> ArgMatches {
        App::new("gcalc")
            .version("0.1.0")
            .author("Simon creek <simoncreek@tutanota.com>")
            .about("Gcalc is a gacha simulator for game development and other decision makings.") // meta information end
            .subcommand(
                App::new("cond")
                .about("Conditional calculation")
                .arg(Arg::new("PROB").about("Basic probability").takes_value(true))
                .arg(Arg::new("reference").about("Reference file").short('r').long("ref").takes_value(true).conflicts_with("refin"))
                .arg(Arg::new("refin").about("Reference from stdin").long("refin").conflicts_with("reference"))
                .arg(Arg::new("budget").about("Budget of total cost").short('b').long("budget").takes_value(true))
                .arg(Arg::new("target").about("Target probability").short('t').long("target").takes_value(true))
                .arg(Arg::new("format").about("Table format").short('f').long("format").takes_value(true))
                .arg(Arg::new("precision").about("Precision").short('p').long("precision").takes_value(true))
                .arg(Arg::new("probtype").about("Probability type").short('T').long("type").takes_value(true))
                .arg(Arg::new("cost").about("Cost per try").short('C').long("cost").takes_value(true))
                .arg(Arg::new("column").about("Column mapping").short('l').long("column").takes_value(true))
                .arg(Arg::new("noheader").about("CSV without header").long("noheader"))
                .arg(Arg::new("out").about("Out file").short('o').long("out").takes_value(true))
            )
            .subcommand(
                App::new("qual")
                .about("Conditional calculation but only prints result")
                .arg(Arg::new("PROB").about("Basic probability").takes_value(true))
                .arg(Arg::new("reference").about("Reference file").short('r').long("ref").takes_value(true).conflicts_with("refin"))
                .arg(Arg::new("refin").about("Reference from stdin").long("refin").conflicts_with("reference"))
                .arg(Arg::new("budget").about("Budget of total cost").short('b').long("budget").takes_value(true))
                .arg(Arg::new("target").about("Target probability").short('t').long("target").takes_value(true))
                .arg(Arg::new("format").about("Table format").short('f').long("format").takes_value(true))
                .arg(Arg::new("precision").about("Precision").short('p').long("precision").takes_value(true))
                .arg(Arg::new("probtype").about("Probability type").short('T').long("type").takes_value(true))
                .arg(Arg::new("cost").about("Cost per try").short('C').long("cost").takes_value(true))
                .arg(Arg::new("column").about("Column mapping").short('l').long("column").takes_value(true))
                .arg(Arg::new("noheader").about("CSV without header").long("noheader"))
                .arg(Arg::new("out").about("Out file").short('o').long("out").takes_value(true))
            )
            .subcommand(
                App::new("range")
                .about("Prints range of calculations")
                .arg(Arg::new("PROB").about("Basic probability").takes_value(true))
                .arg(Arg::new("reference").about("Reference file").short('r').long("ref").takes_value(true).conflicts_with("refin"))
                .arg(Arg::new("refin").about("Reference from stdin").long("refin").conflicts_with("reference"))
                .arg(Arg::new("count").required(true).about("Counts to execute").short('c').long("count").takes_value(true))
                .arg(Arg::new("start").about("Starting index to print").short('s').long("start").takes_value(true))
                .arg(Arg::new("format").about("Table format").short('f').long("format").takes_value(true))
                .arg(Arg::new("precision").about("Precision").short('p').long("precision").takes_value(true))
                .arg(Arg::new("probtype").about("Probability type").short('T').long("type").takes_value(true))
                .arg(Arg::new("cost").about("Cost per try").short('C').long("cost").takes_value(true))
                .arg(Arg::new("column").about("Column mapping").short('l').long("column").takes_value(true))
                .arg(Arg::new("noheader").about("CSV without header").long("noheader"))
                .arg(Arg::new("out").about("Out file").short('o').long("out").takes_value(true))
            ) // "range" subcommand
            .subcommand(App::new("reference")) // "reference" file creation subcommand
            .get_matches()
    }

    fn run_calculator(args: &ArgMatches) -> GcalcResult<()> {
        match args.subcommand() {
            Some(( "range" , range_m)) => {
                Self::subcommand_range(range_m)?;
            }
            Some(( "cond" , cond_m)) => {
                Self::subcommand_conditional(cond_m)?;
            }
            Some(( "qual" , qual_m)) => {
                Self::subcommand_qual(qual_m)?;
            }
            Some(( "reference" , _)) => {
                Self::subcommand_reference()?;
            }
            _ => eprintln!("No proper sub command was given to the program"),
        }

        Ok(())
    }

    fn subcommand_range(args: &ArgMatches) -> GcalcResult<()> {
        let probability = Self::get_sane_probability(args)?;
        let count = args.value_of("count")
            .unwrap()
            .parse()
            .map_err( |_| GcalcError::ParseError("Count should be a positive integer".to_owned()))?;

        let mut cal = Calculator::new(probability)?;
        let mut min = 0;
        if let Some(index) = args.value_of("start") {
            min = index.parse()
                .map_err( |_| GcalcError::ParseError("Start index should be a positive integer (usize)".to_owned()))?;
        }
        Self::set_calculator_attribute(&mut cal, args)?;

        cal.print_range(count, Some(min))?;

        Ok(())
    }

    fn subcommand_conditional(args: &ArgMatches) -> GcalcResult<()> {
        let prob = Self::get_sane_probability(args)?;
        let mut cal = Calculator::new(prob)?;
        Self::set_calculator_attribute(&mut cal, args)?;

        if let Some(target) = args.value_of("target") {
            let prob = target.parse().map_err( |_| GcalcError::ParseError("Target should be a float within 0.0 ~ 1.0".to_owned()))?;
            cal.set_target_probability(prob)?;
        }

        if let Some(budget) = args.value_of("budget") {
            let budget = budget.parse().map_err( |_| GcalcError::ParseError("Budget should be a number".to_owned()))?;
            cal.set_budget(budget);
        }

        cal.print_conditional()?;
        Ok(())
    }

    fn subcommand_qual(args: &ArgMatches) -> GcalcResult<()> {
        let prob = Self::get_sane_probability(args)?;
        let mut cal = Calculator::new(prob)?;
        Self::set_calculator_attribute(&mut cal, args)?;

        if let Some(target) = args.value_of("target") {
            let prob = target.parse().map_err( |_| GcalcError::ParseError("Target should be a float within 0.0 ~ 1.0".to_owned()))?;
            cal.set_target_probability(prob)?;
        }

        if let Some(budget) = args.value_of("budget") {
            let budget = budget.parse().map_err( |_| GcalcError::ParseError("Budget should be a number".to_owned()))?;
            cal.set_budget(budget);
        }

        cal.print_qualfication()?;
        Ok(())
    }

    // Utils, DRY codes

    fn get_sane_probability(args: &ArgMatches) -> GcalcResult<f32> {
        let probability = args.value_of("PROB")
            .unwrap_or("1.0")
            .parse()
            .map_err( |_| GcalcError::ParseError("Probability should be a float within 0.0 ~ 1.0".to_owned()))?;
        utils::prob_sanity_check(probability)?;
        Ok(probability)
    }

    fn set_calculator_attribute(cal : &mut Calculator, args: &ArgMatches) -> GcalcResult<()> {
        if let Some(cost) = args.value_of("cost") {
            let cost = cost.parse().map_err( |_| GcalcError::ParseError("Cost should be a number".to_owned()))?;
            cal.set_cost(cost);
        }

        // Reference and refin is mutual exclusive
        if let Some(csv_file) = args.value_of("reference") {
            cal.set_csv_file(RefCsv::File(PathBuf::from(csv_file)));
        } else if args.is_present("refin") {
            let stdin = std::io::stdin();
            let mut string = String::new();
            stdin.lock().read_to_string(&mut string)?;
            cal.set_csv_file(RefCsv::Raw(string));
        }

        if let Some(format) = args.value_of("format") {
            cal.set_table_format(TableFormat::from_str(format)?);
        }

        if let Some(precision) = args.value_of("precision") {
            let precision = precision.parse()
                .map_err( |_| GcalcError::ParseError("Precision should be a positive integer (usize)".to_owned()))?;
            cal.set_precision(precision);
        }

        if let Some(prob_type) = args.value_of("probtype") {
            cal.set_prob_type(ProbType::from_str(prob_type)?);
        }

        if let Some(file) = args.value_of("out") {
            cal.set_out_file(std::path::Path::new(file));
        }

        Self::set_custom_column_order(cal, args)?;

        // No header
        if args.is_present("noheader") {
            cal.set_no_header(true);
        }

        Ok(())
    }

    fn set_custom_column_order(cal : &mut Calculator, args: &ArgMatches) -> GcalcResult<()> {
        if let Some(order) = args.value_of("column") {
            let split_orders = order.split(',').collect::<Vec<&str>>();
            if split_orders.len() < COLUMN_LEN {
                return Err(GcalcError::InvalidArgument(format!("Column's length should be bigger or equal to \"{}\"", COLUMN_LEN)));
            }
            let (mut count, mut probability, mut added, mut cost) = (COUNT_INDEX, BASIC_PROB_INDEX, ADDED_PROB_INDEX, COST_INDEX);
            for (index, &item) in split_orders.iter().enumerate() {
                match item {
                    "count" => count = index,
                    "probability" => probability = index,
                    "added" => added = index,
                    "cost" => cost = index,
                    _ => (), // Skip item
                }
            } 
            cal.set_column_map(ColumnMap::new(count, probability, added, cost));
        }
        Ok(())
    }

    fn subcommand_reference() -> GcalcResult<()> {
        std::fs::write(std::path::Path::new("ref.csv"), r#"count,probability,bonus,cost"#)?;
        Ok(())
    }
}
