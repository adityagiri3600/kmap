use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "kmap")]
#[command(about = "A command line tool to process k-maps", long_about = None)]
pub struct Cli {
    /// Number of variables in the k-map
    pub number_of_variables: u32,

    /// List of minterms
    #[arg(required = true)]
    pub minterms: Vec<u32>,
}