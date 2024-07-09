mod group;
mod kmap;
mod cli;
use clap::Parser;
use cli::Cli;
use kmap::Kmap;

fn main() {
    let args = Cli::parse();
    let kmap = Kmap::new(args.number_of_variables, args.minterms);
    kmap.print_solution();
}
