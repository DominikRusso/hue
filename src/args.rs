use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Subcommand {
    /// For pairing with the hue bridge on the network
    Init { username: String },

    /// For turning lights on
    ///
    /// Turns the specified lights on. If no lights are specified all lights
    /// are turned on.
    On { lights: Vec<String> },

    /// For turning lights off
    ///
    /// Turns the specified lights off. If no lights are specified all lights
    /// are turned off.
    Off { lights: Vec<String> },

    /// For controlling lights' brightness
    ///
    /// Sets the brightness of the specified lights. If no lights are specified
    /// it sets the brightness of all lights.
    #[structopt(alias = "bri", settings = &[AppSettings::AllowNegativeNumbers])]
    Brightness {
        brightness: String,
        lights: Vec<String>,
    },

    // TODO
    /// For controlling lights' color (unimplemented)
    ///
    /// Sets the color of the specified lights. If no lights are specified it
    /// sets the color of all lights.
    #[structopt(alias = "col")]
    Color { color: String, lights: Vec<String> },

    /// For setting scenes
    #[structopt(alias = "set")]
    Scene { name: String },
}
