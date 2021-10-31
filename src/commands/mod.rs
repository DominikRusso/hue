use huelib::bridge::Bridge;
use huelib::resource::{light::StateModifier, Adjust};
use std::env;
use std::fs::{self, File};
use std::net::IpAddr;
use xdg::BaseDirectories;

static ENV_IP: &str = "HUE_IP";
static ENV_USER: &str = "HUE_USER";

pub fn init(username: &str) -> Result<(), huelib::Error> {
    // damn this library's api is ugly
    use huelib::response::Error as ResponseError;
    use huelib::response::ErrorKind::LinkButtonNotPressed;
    use huelib::Error;

    let mut info_msg_printed = false;

    // check first if the user specified the IP and only then use
    // `discover_nupnp` which connects to Philip's severs
    // NOTE: I wonder if something like avahi could be used instead
    let ip = match env::var(ENV_IP) {
        Ok(ip) => ip.parse::<IpAddr>().unwrap(),
        Err(_) => huelib::bridge::discover_nupnp()?
            .pop()
            .expect("No bridges found."),
    };

    // loop until bridge button has been pressed
    loop {
        match huelib::bridge::register_user(ip, &username) {
            Ok(user) => {
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

pub enum PowerState {
    On,
    Off,
}

impl From<PowerState> for bool {
    fn from(ps: PowerState) -> bool {
        match ps {
            PowerState::On => true,
            PowerState::Off => false,
        }
    }
}

pub fn power(lights: Vec<String>, power_state: PowerState) {
    let state_transform = StateModifier::new().with_on(power_state.into());
    apply_transform(lights, state_transform);
}

pub fn brightness(lights: Vec<String>, brightness: String) {
    let prefix;
    let value;
    if brightness.starts_with("+") || brightness.starts_with("-") {
        prefix = Some(brightness.chars().next().unwrap());
        value = brightness[1..].to_string();
    } else {
        prefix = None;
        value = brightness;
    }

    let value = value.parse::<u8>().expect("Failed to parse brightness.");
    let value = ((value as f32 / 100.0) * 255.0) as u8;

    let brightness_transform = match prefix {
        Some('+') => Adjust::Increment(value),
        Some('-') => Adjust::Decrement(value),
        None => Adjust::Override(value),
        _ => unreachable!(),
    };

    let state_transform = StateModifier::new()
        .with_on(true)
        .with_brightness(brightness_transform);
    apply_transform(lights, state_transform);
}

fn apply_transform(lights: Vec<String>, state_transform: StateModifier) {
    let bridge = login();
    if lights.is_empty() {
        // apply transform to all lights
        let lights = bridge.get_all_lights().expect("Failed to get lights.");
        for light in lights {
            dbg!(bridge.set_light_state(light.id, &state_transform).unwrap());
        }
    } else {
        // apply transform only to specified lights
        unimplemented!()
    }
}

fn login() -> Bridge {
    let xdg_dirs = BaseDirectories::with_prefix("hue").unwrap();
    let ip = match env::var(ENV_IP) {
        Ok(ip) => ip
            .parse::<IpAddr>()
            .expect("Failed to parse IP address on `HUE_IP` environment variable."),
        Err(_) => {
            let bridge_path = xdg_dirs
                .find_data_file("bridge")
                .expect("Cannot find IP address of bridge in environment variable `HUE_IP` and the bridge file does not exist.");
            fs::read_to_string(bridge_path)
                .unwrap()
                .parse()
                .expect("Failed to parse IP address in username data file.")
        }
    };

    let username = match env::var(ENV_USER) {
        Ok(user) => user,
        Err(_) => {
            let username_path = xdg_dirs
                .find_data_file("username")
                .expect("Cannot find username in environment variable `HUE_USER` and the username file does not exist.");
            fs::read_to_string(username_path).unwrap()
        }
    };

    Bridge::new(ip, username)
}
