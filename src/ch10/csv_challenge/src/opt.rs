use structopt_derive::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "csv_challenge", about = "An example of StructOpt usage.")]
pub struct Opt {
    /// Needed parameter, the first on the command line.
    // #[structopt(short = "v", long = "verbose")]
    // pub verbosity: u64,
    #[structopt(help = "Input file")]
    pub input: String,
    #[structopt(help = "Column Name")]
    pub column_name: String,
    #[structopt(help = "Replacement Column Name")]
    pub replacement: String,
    #[structopt(help = "Output file, stdout if not present")]
    pub output: Option<String>,
}