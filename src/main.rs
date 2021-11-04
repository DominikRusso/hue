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
                panic!("{}", e);
            }
        }
        Subcommand::On { lights } => {
            commands::power(lights, PowerState::On);
        }
        Subcommand::Off { lights } => {
            commands::power(lights, PowerState::Off);
        }
        Subcommand::Brightness { brightness, lights } => {
            commands::brightness(lights, brightness);
        }
        Subcommand::Color { color: _, lights: _ } => {
            unimplemented!()
        }
        Subcommand::Scene { name } => {
            commands::scene(name);
        }
    };
}
