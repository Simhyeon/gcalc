use crate::{GcalcResult, Calculator, utils, TableFormat, ProbType, models::{CsvRef, ColumnMap}, GcalcError, calc::CalculatorOption};
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

        let cond_app  = Self::common_app_args(App::new("cond").about("Conditional calculation"));
        let range_app = Self::common_app_args(App::new("range").about("Prints range of calculations"));
        let qual_app  = Self::common_app_args(App::new("qual").about("Conditional calculation but only prints result"));

        let main_app = App::new("gcalc")
            .version("0.2.0")
            .author("Simon creek <simoncreek@tutanota.com>")
            .about("Gcalc is a gacha simulator for game development and other decision makings.") // meta information end
            .subcommand(cond_app
                .arg(Arg::new("budget").help("Budget of total cost").short('b').long("budget").takes_value(true))
                .arg(Arg::new("target").help("Target probability").short('t').long("target").takes_value(true))
                .arg(Arg::new("offset").help("Record offset").long("offset").takes_value(true))
            ).subcommand(qual_app
                .arg(Arg::new("budget").help("Budget of total cost").short('b').long("budget").takes_value(true))
                .arg(Arg::new("target").help("Target probability").short('t').long("target").takes_value(true))
            ).subcommand(range_app 
                .arg(Arg::new("count").help("Counts to execute").short('c').long("count").takes_value(true))
                .arg(Arg::new("start").help("Starting index to print").short('S').long("start").takes_value(true))
            ).subcommand(App::new("reference").about("Create a reference file"));

            #[cfg(feature = "option")]
            let app = main_app.subcommand(App::new("option").about("Create an option file")); // "option" file creation subcommand

            app.get_matches()
    }

    fn common_app_args(app : clap::App) -> clap::App {
        let app = app.arg(Arg::new("prob").help("Basic probability").short('p').long("probability").takes_value(true))
            .arg(Arg::new("cost").help("Cost per try").short('C').long("cost").takes_value(true))
            .arg(Arg::new("constant").help("Constant value to be added into probability").long("constant").takes_value(true))
            .arg(Arg::new("reference").help("Reference file").short('r').long("ref").takes_value(true).conflicts_with("refin"))
            .arg(Arg::new("refin").help("Reference from stdin").long("refin").conflicts_with("reference"))
            .arg(Arg::new("format").help("Table format(csv|console|gfm)").short('f').long("format").takes_value(true))
            .arg(Arg::new("precision").help("Precision").short('P').long("precision").takes_value(true))
            .arg(Arg::new("probtype").help("Probability type(percentage|fraction)").short('T').long("type").takes_value(true))
            .arg(Arg::new("column").help("Column mapping").short('l').long("column").takes_value(true))
            .arg(Arg::new("noheader").help("CSV without header").long("noheader"))
            .arg(Arg::new("out").help("Out file").short('o').long("out").takes_value(true))
            .arg(Arg::new("fallback").help("Set csv value fallback (rollback|ignore|none)").long("fallback").default_value("none"))
            .arg(Arg::new("strict").help("Set strict CSV reader mode").short('s').long("strict"));

        #[cfg(feature = "option")]
        let app = app.arg(Arg::new("option").help("Option file to use").short('O').long("option").takes_value(true));

        app
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
            #[cfg(feature = "option")]
            Some(( "option" , _)) => {
                Self::subcommand_option()?;
            }
            _ => eprintln!("No proper sub command was given to the program"),
        }

        Ok(())
    }

    fn subcommand_range(args: &ArgMatches) -> GcalcResult<()> {
        // Override count if value was given
        let count = if let Some(count) = args.value_of("count") {
            Some(count.parse::<usize>().map_err( |_| GcalcError::ParseError("Count should be a positive integer".to_owned()))?)
        } else { None };

        let mut cal = Calculator::new()?;
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
        let mut cal = Calculator::new()?;
        Self::set_calculator_attribute(&mut cal, args)?;

        if let Some(target) = args.value_of("target") {
            let prob = target.parse().map_err( |_| GcalcError::ParseError("Target should be a float within 0.0 ~ 1.0".to_owned()))?;
            cal.set_target_probability(prob)?;
        }

        if let Some(budget) = args.value_of("budget") {
            let budget = budget.parse().map_err( |_| GcalcError::ParseError("Budget should be a number".to_owned()))?;
            cal.set_budget(budget);
        }

        if let Some(offset) = args.value_of("offset") {
            let offset = offset.parse().map_err( |_| GcalcError::ParseError("Budget should be a number".to_owned()))?;
            cal.set_offset(offset);
        }

        cal.print_conditional()?;
        Ok(())
    }

    fn subcommand_qual(args: &ArgMatches) -> GcalcResult<()> {
        let mut cal = Calculator::new()?;
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

    fn set_calculator_attribute(cal : &mut Calculator, args: &ArgMatches) -> GcalcResult<()> {
        #[cfg(feature = "option")]
        if let Some(file) = args.value_of("option") {
            let option =  CalculatorOption::from_file(std::path::Path::new(file))?;
            cal.set_option(&option);
        }

        if let Some(prob) = args.value_of("prob") {
            let probability = utils::get_prob_alap(prob, None)?;
            cal.set_probability(probability, true)?;
        }

        if let Some(cost) = args.value_of("cost") {
            let cost = cost.parse().map_err( |_| GcalcError::ParseError("Cost should be a number".to_owned()))?;
            cal.set_cost(cost, true);
        }

        if let Some(cost) = args.value_of("constant") {
            let constant = cost.parse().map_err( |_| GcalcError::ParseError("Constant should be a number".to_owned()))?;
            cal.set_constant(constant, true)?;
        }

        // Reference and refin is mutual exclusive
        if let Some(csv_file) = args.value_of("reference") {
            cal.set_csv_file(CsvRef::File(PathBuf::from(csv_file)));
        } else if args.is_present("refin") {
            let stdin = std::io::stdin();
            let mut string = String::new();
            stdin.lock().read_to_string(&mut string)?;
            cal.set_csv_file(CsvRef::Raw(string));
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

        cal.set_csv_value_fallback(args.value_of("fallback").unwrap_or("none"))?;

        cal.set_strict_csv(args.is_present("strict"));

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
            let (mut count, mut probability, mut constant, mut cost) = (COUNT_INDEX, BASIC_PROB_INDEX, ADDED_PROB_INDEX, COST_INDEX);
            for (index, &item) in split_orders.iter().enumerate() {
                match item {
                    "count" => count = index,
                    "probability" => probability = index,
                    "constant" => constant = index,
                    "cost" => cost = index,
                    _ => (), // Skip item
                }
            } 
            cal.set_column_map(ColumnMap::new(count, probability, constant, cost));
        }
        Ok(())
    }

    fn subcommand_reference() -> GcalcResult<()> {
        std::fs::write(std::path::Path::new("ref.csv"), r#"count,probability,constant,cost"#)?;
        Ok(())
    }

    #[cfg(feature = "option")]
    fn subcommand_option() -> GcalcResult<()> {
        let path = std::path::Path::new("option.json");
        println!("Created new option file \"{}\"", path.display());
        std::fs::write(path, CalculatorOption::new().to_json()?)?;
        Ok(())
    }
}
