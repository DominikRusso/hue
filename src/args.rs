use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(author)]
pub enum Subcommand {
    /// For pairing with the hue bridge on the network
    Init { username: String },

    /// For turning lights on
    ///
    /// Turns the specified lights on.
    /// If no lights are specified all lights are turned on.
    #[structopt(verbatim_doc_comment)]
    On { lights: Vec<String> },

    /// For turning lights off
    ///
    /// Turns the specified lights off.
    /// If no lights are specified all lights are turned off.
    #[structopt(verbatim_doc_comment)]
    Off { lights: Vec<String> },

    /// For controlling lights' brightness
    ///
    /// Sets the brightness of the specified lights.
    /// If no lights are specified it sets the brightness of the lights that are on.
    /// If you want to turn all the lights on and set their brightness in one command
    /// you can pass `-a` or `--all`.
    ///
    /// To make relative brightness changes you can prefix the brightness value with
    /// `+` or `-`.
    #[structopt(alias = "bri", settings = &[AppSettings::AllowNegativeNumbers])]
    #[structopt(verbatim_doc_comment)]
    Brightness {
        brightness: String,
        lights: Vec<String>,
        #[structopt(short, long, conflicts_with = "lights")]
        all: bool,
    },

    /// For controlling lights' color
    ///
    /// Sets the color of the specified lights.
    /// If no lights are specified it sets the color of the lights that are on.
    /// If you want to turn all the lights on and set their color in one command you can
    /// pass `-a` or `--all`.
    #[structopt(alias = "col")]
    #[structopt(verbatim_doc_comment)]
    Color {
        color: String,
        lights: Vec<String>,
        #[structopt(short, long, conflicts_with = "lights")]
        all: bool,
    },

    /// For setting scenes
    #[structopt(alias = "set")]
    Scene { name: String },
}
