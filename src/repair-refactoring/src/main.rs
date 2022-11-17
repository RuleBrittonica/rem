extern crate colored;

use std::iter::zip;
use std::process::exit;
use std::time::SystemTime;
use colored::Colorize;
use clap::{Parser, ValueEnum, Subcommand};

mod common;
mod repair_lifetime_simple;
mod repair_rustfix;
mod repair_lifetime_tightest_bound_first;
mod repair_lifetime_loosest_bound_first;

use common::RepairSystem;
use crate::RepairerType::{LoosestBoundsFirst, TightestBoundsFirst};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the repairs
    Run {
        fn_sig: String,
        file_name: String,
        new_file_name: String,
        repairer: RepairerType,
        #[arg(short, long)]
        verbose: bool,
    },
    /// Test all repair systems against inputs in ./input
    Test {},
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum RepairerType {
    Simple,
    LoosestBoundsFirst,
    TightestBoundsFirst,
}

fn main () {
    let args = Cli::parse();
    match &args.command {
        Commands::Test {} => { test() },
        Commands::Run {fn_sig, repairer, file_name, new_file_name, verbose} => {
            let repair_system: &dyn RepairSystem =
                match repairer {
                    RepairerType::Simple => &repair_lifetime_simple::Repairer {},
                    TightestBoundsFirst => &repair_lifetime_tightest_bound_first::Repairer {},
                    LoosestBoundsFirst => &repair_lifetime_loosest_bound_first::Repairer {},
                };
            let success =
                if *verbose {
                    print_repair_stat(
                        &repair_system,
                        file_name,
                        new_file_name,
                        fn_sig,
                    )
                } else {
                    repair_system.repair_function(
                        file_name,
                        new_file_name,
                        fn_sig,
                        fn_sig,
                    )
                };
            if !success {
                exit(1)
            }
        }
    }
}

fn print_repair_stat(repair_system: &&dyn RepairSystem, file_name: &str, new_file_name: &str, function_sig: &str) -> bool {
    println!("\n\n{}: {}", file_name, function_sig);
    let now = SystemTime::now();
    let success =
        repair_system.repair_function(
            file_name,
            new_file_name,
            function_sig,
            function_sig,
        );
    let time_elapsed = now.elapsed().unwrap();
    println!("{}: {} refactored {} in {:#?}",
             (if success { format!("PASSED").green() } else { format!("FAILED").red() }), repair_system.name(), file_name, time_elapsed);
    success
}

fn test() {
    let file_names = vec!["borrow", "in_out_lifetimes", "lifetime_bounds", "in_out_lifetimes_original_extract", "lifetime_bounds_not_enough_annotations", "in_out_lifetimes_wide_bounds"];
    let function_sigs = vec![("",""), ("bar_extracted", "fn bar_extracted(x_ref: &i32, z: &i32, y: &i32) -> &i32"), ("bar_extracted", "fn bar_extracted(p: &mut & i32, x: & i32)"), ("", ""), ("", ""), ("bar_extracted", "fn bar_extracted<'a, 'b, 'c>(x_ref: &'a i32, z: &'b i32, y: &'c i32) -> &'a i32 {")];
    let repair_systems: Vec<&dyn RepairSystem> = vec![&repair_lifetime_simple::Repairer {}, &repair_rustfix::Repairer {}, &repair_lifetime_tightest_bound_first::Repairer {}, &repair_lifetime_loosest_bound_first::Repairer {}];
    for (file_name, (_, function_sig)) in zip(file_names, function_sigs) {
        for repair_system in repair_systems.iter() {
            let new_file_name = format!("./output/{}{}.rs", file_name, repair_system.name());
            let file_name = format!("./input/{}.rs", file_name);
            print_repair_stat(repair_system, file_name.as_str(), new_file_name.as_str(), function_sig);
        }
        println!("------------------------------------------------------------------");
    }
}