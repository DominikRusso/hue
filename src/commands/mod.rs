use huelib::bridge::Bridge;
use huelib::resource::{group, light, scene::Scene, Adjust};
use huelib::response::Error as ResponseError;
use huelib::response::ErrorKind::LinkButtonNotPressed;
use huelib::Error;
use std::env;
use std::fs::{self, File};
use std::net::IpAddr;
use std::num;
use xdg::BaseDirectories;

static ENV_IP: &str = "HUE_IP";
static ENV_USER: &str = "HUE_USER";

pub fn init(username: &str) -> Result<(), huelib::Error> {
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
                    println!("Press the bridge's link button!");
                }
                info_msg_printed = true;
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            Err(e) => return Err(e),
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
    fn from(ps: PowerState) -> Self {
        match ps {
            PowerState::On => true,
            PowerState::Off => false,
        }
    }
}

pub fn power(lights: Vec<String>, power_state: PowerState) {
    let state_transform = light::StateModifier::new().with_on(power_state.into());
    if let Err(e) = apply_transform(lights, state_transform, true) {
        eprintln!("{}", e);
    };
}

pub fn brightness(lights: Vec<String>, brightness: String, all: bool) {
    let (prefix, value) = if brightness.starts_with('+') || brightness.starts_with('-') {
        (
            Some(brightness.chars().next().unwrap()),
            brightness[1..].to_string(),
        )
    } else {
        (None, brightness)
    };

    let value = match value.parse::<u8>() {
        Ok(v) => v,
        Err(e) if e.kind() == &num::IntErrorKind::PosOverflow => {
            eprintln!("The brightness value must be between 0 and 100.");
            return;
        }
        Err(_) => {
            eprintln!("The brightness value must be of the form `[+-]<int>`.");
            return;
        }
    };
    let value = ((value as f32 / 100.0) * 255.0) as u8;

    let brightness_transform = match prefix {
        Some('+') => Adjust::Increment(value),
        Some('-') => Adjust::Decrement(value),
        None => Adjust::Override(value),
        _ => unreachable!(),
    };

    let state_transform = light::StateModifier::new()
        .with_on(true)
        .with_brightness(brightness_transform);
    if let Err(e) = apply_transform(lights, state_transform, all) {
        eprintln!("{}", e);
    };
}

pub fn color(color: String, lights: Vec<String>, all: bool) {
    let color = match pastel::parser::parse_color(&color) {
        Some(c) => c,
        None => {
            eprintln!("Color couldn't be parsed.");
            return;
        }
    };

    let cie_xyz = color.to_xyz();
    let x = cie_xyz.x as f32;
    let y = cie_xyz.y as f32;
    let z = cie_xyz.z as f32;
    // https://en.wikipedia.org/wiki/CIE_1931_color_space
    let color_space_coordinates = (
        x / (x + y + z + f32::MIN_POSITIVE),
        y / (x + y + z + f32::MIN_POSITIVE),
    );

    let state_transform = light::StateModifier::new()
        .with_on(true)
        .with_color_space_coordinates(Adjust::Override(color_space_coordinates));
    if let Err(e) = apply_transform(lights, state_transform, all) {
        eprintln!("{}", e);
    };
}

pub fn scene(name: String) -> Result<(), String> {
    let bridge = login()?;
    let scenes = bridge.get_all_scenes().unwrap();
    let filtered_scenes: Vec<&Scene> = scenes
        .iter()
        .filter(|sc| sc.name == name && sc.group.is_some())
        .collect();
    let target_scene = filtered_scenes
        .first()
        .ok_or("No scene with that name found.")?;
    let group_id = target_scene
        .group
        .as_ref()
        .expect("Bug: Filtered scenes should all have a group.");
    let scene_id = &target_scene.id;

    let state_transform = group::StateModifier::new()
        .with_on(true)
        .with_scene(scene_id.to_string());
    bridge.set_group_state(group_id, &state_transform).unwrap();

    Ok(())
}

fn apply_transform(
    lights: Vec<String>,
    state_transform: light::StateModifier,
    apply_to_all: bool,
) -> Result<(), String> {
    let bridge = login()?;
    let all_lights = bridge.get_all_lights().expect("Failed to get lights.");
    if lights.is_empty() {
        if apply_to_all {
            // apply transform to all lights
            for light in all_lights {
                bridge.set_light_state(light.id, &state_transform).unwrap();
            }
        } else {
            // apply to lights that are on
            let active_lights: Vec<&light::Light> = all_lights
                .iter()
                .filter(|light| light.state.on.is_some() && light.state.on.unwrap())
                .collect();
            for light in active_lights {
                bridge.set_light_state(&light.id, &state_transform).unwrap();
            }
        }
    } else {
        // apply transform only to specified lights

        // make sure the specified lights all exist
        if !lights.iter().all(|input| {
            all_lights
                .iter()
                .map(|a| &a.name)
                .any(|light| light == input)
        }) {
            return Err("One of the specified lights was not found.".into());
        }
        for light in all_lights {
            if lights.contains(&light.name) {
                bridge.set_light_state(light.id, &state_transform).unwrap();
            }
        }
    }
    Ok(())
}

fn login() -> Result<Bridge, String> {
    let xdg_dirs = BaseDirectories::with_prefix("hue").unwrap();
    let ip = match env::var(ENV_IP) {
        Ok(ip) => ip.parse::<IpAddr>().or(Err(
            "Failed to parse IP address on `HUE_IP` environment variable.",
        ))?,
        Err(_) => {
            let bridge_path = xdg_dirs
                .find_data_file("bridge")
                .ok_or("Cannot find IP address of bridge in environment variable `HUE_IP` and the bridge file does not exist.")?;
            fs::read_to_string(bridge_path)
                .unwrap()
                .parse()
                .or(Err("Failed to parse IP address in username data file."))?
        }
    };

    let username = match env::var(ENV_USER) {
        Ok(user) => user,
        Err(_) => {
            let username_path = xdg_dirs
                .find_data_file("username")
                .ok_or("Cannot find username in environment variable `HUE_USER` and the username file does not exist.")?;
            fs::read_to_string(username_path).unwrap()
        }
    };

    Ok(Bridge::new(ip, username))
}
