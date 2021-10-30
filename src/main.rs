mod args;

use args::Subcommand;
use std::env;
use std::fs::File;
use structopt::StructOpt;
use xdg::BaseDirectories;

fn main() {
    let subcommand = Subcommand::from_args();
    dbg!(&subcommand);

    match subcommand {
        Subcommand::Init { username } => {
            let _ = init(&username);
        }
        Subcommand::On { lights } => {}
        Subcommand::Off { lights } => {}
        Subcommand::Brightness { brightness, lights } => {}
        Subcommand::Color { color, lights } => {}
        Subcommand::Scene { name } => {}
    };
}

fn init(username: &str) -> Result<(), huelib::Error> {
    use std::net::IpAddr;
    // damn this library's api is ugly
    use huelib::response::Error as ResponseError;
    use huelib::response::ErrorKind::LinkButtonNotPressed;
    use huelib::Error;

    let mut info_msg_printed = false;

    // check first if the user specified the IP and only then use
    // `discover_nupnp` which connects to Philip's severs
    // NOTE: I wonder if something like avahi could be used instead
    let ip = match env::var("HUE_IP") {
        Ok(ip) => ip.parse::<IpAddr>().unwrap(),
        Err(_) => huelib::bridge::discover_nupnp()?
            .pop()
            .expect("No bridges found."),
    };

    // loop until bridge button has been pressed
    loop {
        match huelib::bridge::register_user(ip, &username) {
            Ok(user) => {
                dbg!(&user);
                let xdg_dirs = BaseDirectories::with_prefix("hue").unwrap();
                let username_path = xdg_dirs
                    .place_data_file("username")
                    .expect("Couldn't create data directory.");
                let bridge_path = xdg_dirs
                    .place_data_file("bridge")
                    .expect("Couldn't create data directory.");
                let mut username_file = File::create(username_path)?;
                let mut bridge_file = File::create(bridge_path)?;
                use std::io::Write;
                write!(username_file, "{}", user).unwrap();
                write!(bridge_file, "{}", ip).unwrap();
                break;
            }
            Err(Error::Response(ResponseError { kind, .. })) if kind == LinkButtonNotPressed => {
                if !info_msg_printed {
                    println!("Press the bridge button!");
                }
                info_msg_printed = true;
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
    println!("Successfully paired with the bridge.");
    Ok(())
}

// fn check_initialized() -> bool {}
