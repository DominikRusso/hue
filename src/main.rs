mod args;

use structopt::StructOpt;

fn main() {
    let subcommand = args::Subcommand::from_args();
    println!("{:?}", subcommand);
}
