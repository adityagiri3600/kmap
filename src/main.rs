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
        println!("\x1b[104m{}\x1b[0m",kmap.expression());
    }
    let duration = start.elapsed();
    println!(
        "\nTotal \x1b[91m{}\x1b[0m groups, found solution out of \x1b[91m10^{:0.2}\x1b[0m possible",
        3_usize.pow(args.number_of_variables as u32),
        (3_usize.pow(args.number_of_variables as u32) as f64) * 0.30102999566
    );
    println!("Took \x1b[91m{:?}\x1b[0m", duration);
}
