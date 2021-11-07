mod args;
mod commands;

use args::Subcommand;
use commands::PowerState;
use structopt::StructOpt;

fn main() {
    let subcommand = Subcommand::from_args();

    match subcommand {
        Subcommand::Init { username } => {
            if let Err(e) = commands::init(&username) {
                eprintln!("{}", e);
            }
        }
        Subcommand::On { lights } => {
            commands::power(lights, PowerState::On);
        }
        Subcommand::Off { lights } => {
            commands::power(lights, PowerState::Off);
        }
        Subcommand::Brightness {
            brightness,
            lights,
            all,
        } => {
            commands::brightness(lights, brightness, all);
        }
        Subcommand::Color { color, lights, all } => {
            commands::color(color, lights, all);
        }
        Subcommand::Scene { name } => {
            if let Err(e) = commands::scene(name) {
                eprintln!("{}", e);
            }
        }
    };
}
