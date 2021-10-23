use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Subcommand {
    Pair {
        username: String,
    },
    On {
        lights: Vec<String>,
    },
    Off {
        lights: Vec<String>,
    },
    #[structopt(alias = "bri")]
    Brightness {
        brightness: u8,
        lights: Vec<String>,
    },
    #[structopt(alias = "col")]
    Color {
        color: String,
        lights: Vec<String>,
    },
    #[structopt(alias = "set")]
    Scene {
        name: String,
    },
}
