mod cli;
mod group;
mod kmap;
use clap::Parser;
use cli::Cli;
use kmap::Kmap;

fn main() {
    let args = Cli::parse();
    use std::time::Instant;
    let start = Instant::now();
    {
        let kmap = Kmap::new(args.number_of_variables, args.minterms);
        println!();
        kmap.print_solution();
    }
    let duration = start.elapsed();
    println!(
        "\nTotal {} groups, found solution out of 10^{:0.2} possible",
        3_usize.pow(args.number_of_variables as u32),
        (3_usize.pow(args.number_of_variables as u32) as f64) * 0.30102999566
    );
    println!("Took {:?}", duration);
}
