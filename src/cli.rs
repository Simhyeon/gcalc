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
        App::new("gcalc")
            .version("0.1.0")
            .author("Simon creek <simoncreek@tutanota.com>")
            .about("Gcalc is a gacha simulator for game development and other decision makings.") // meta information end
            .subcommand(
                App::new("cond")
                .about("Conditional calculation")
                .arg(Arg::new("PROB").about("Basic probabilty").takes_value(true))
                .arg(Arg::new("reference").about("Reference file").short('r').long("ref").takes_value(true))
                .arg(Arg::new("budget").about("Budget of total cost").short('b').long("budget").takes_value(true))
                .arg(Arg::new("target").about("Target probabilty").short('t').long("target").takes_value(true))
                .arg(Arg::new("format").about("Table format").short('f').long("format").takes_value(true))
                .arg(Arg::new("precision").about("Precision").short('p').long("precision").takes_value(true))
                .arg(Arg::new("probtype").about("Probabilty type").short('T').long("type").takes_value(true))
                .arg(Arg::new("cost").about("Cost per try").short('C').long("cost").takes_value(true))
            )
            .subcommand(
                App::new("qual")
                .about("Conditional calculation but only prints result")
                .arg(Arg::new("PROB").about("Basic probabilty").takes_value(true))
                .arg(Arg::new("reference").about("Reference file").short('r').long("ref").takes_value(true))
                .arg(Arg::new("budget").about("Budget of total cost").short('b').long("budget").takes_value(true))
                .arg(Arg::new("target").about("Target probabilty").short('t').long("target").takes_value(true))
                .arg(Arg::new("format").about("Table format").short('f').long("format").takes_value(true))
                .arg(Arg::new("precision").about("Precision").short('p').long("precision").takes_value(true))
                .arg(Arg::new("probtype").about("Probabilty type").short('T').long("type").takes_value(true))
                .arg(Arg::new("cost").about("Cost per try").short('C').long("cost").takes_value(true))
            )
            .subcommand(
                App::new("range")
                .about("Prints range of calculations")
                .arg(Arg::new("PROB").about("Basic probabilty").takes_value(true))
                .arg(Arg::new("reference").about("Reference file").short('r').long("ref").takes_value(true))
                .arg(Arg::new("count").required(true).about("Counts to execute").short('c').long("count").takes_value(true))
                .arg(Arg::new("start").about("Starting index to print").short('s').long("start").takes_value(true))
                .arg(Arg::new("format").about("Table format").short('f').long("format").takes_value(true))
                .arg(Arg::new("precision").about("Precision").short('p').long("precision").takes_value(true))
                .arg(Arg::new("probtype").about("Probabilty type").short('T').long("type").takes_value(true))
                .arg(Arg::new("cost").about("Cost per try").short('C').long("cost").takes_value(true))
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
            .expect("count should be an integer");

        let mut cal = Calculator::new(probability)?;
        let mut min = 0;
        if let Some(index) = args.value_of("start") {
            min = index.parse().expect("Failed to get precisino as usize");
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
            cal.set_target_probability(target.parse().expect("Failed to get target prob"))?;
        }

        if let Some(budget) = args.value_of("budget") {
            cal.set_budget(budget.parse().expect("Failed to get budget"));
        }

        cal.print_conditional()?;
        Ok(())
    }

    fn subcommand_qual(args: &ArgMatches) -> GcalcResult<()> {
        let prob = Self::get_sane_probability(args)?;
        let mut cal = Calculator::new(prob)?;
        Self::set_calculator_attribute(&mut cal, args)?;

        if let Some(target) = args.value_of("target") {
            cal.set_target_probability(target.parse().expect("Failed to get target prob"))?;
        }

        if let Some(budget) = args.value_of("budget") {
            cal.set_budget(budget.parse().expect("Failed to get budget"));
        }

        cal.print_qualfication()?;
        Ok(())
    }

    // Utils, DRY codes

    fn get_sane_probability(args: &ArgMatches) -> GcalcResult<f32> {
        let probabilty = args.value_of("PROB")
            .unwrap_or("1.0")
            .parse()
            .expect("Probabilty should be float");
        utils::prob_sanity_check(probabilty)?;
        Ok(probabilty)
    }

    fn set_calculator_attribute(cal : &mut Calculator, args: &ArgMatches) -> GcalcResult<()> {
        if let Some(cost) = args.value_of("cost") {
            cal.set_cost(cost.parse().expect("Failed to get cost"));
        }

        if let Some(csv_file) = args.value_of("reference") {
            cal.set_csv_file(std::path::Path::new(csv_file));
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
        Ok(())
    }

    fn subcommand_reference() -> GcalcResult<()> {
        std::fs::write(std::path::Path::new("ref.csv"), r#"count,probabilty,bonus,cost"#)?;
        Ok(())
    }
}
