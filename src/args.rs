use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Subcommand {
    Init {
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
        brightness: String,
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
