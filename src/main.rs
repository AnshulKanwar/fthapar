use clap::Parser;

use fthapar::Config;

#[derive(Parser, Debug)]
struct Cli {
    /// Check password for a single enrollment number
    enrollment_number: u32,

    /// To verify for a range of enrollment numbers also give and ending enrollment number
    #[clap(short, long)]
    last_enrollent_number: Option<u32>,

    /// Give a password/pin to be checked agaist
    #[clap(short, long, default_value_t = String::from("12345"))]
    password: String
}

fn main() {
    let cli = Cli::parse();

    let config = Config {
        enrollment_number_start: cli.enrollment_number,
        enrollment_number_end: cli.last_enrollent_number,
        password: cli.password
    };

    fthapar::run(config);
}
