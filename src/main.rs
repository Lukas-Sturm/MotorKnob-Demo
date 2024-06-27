mod motorknob;

use gpiod::{Bias, Chip, EdgeDetect, Options, Edge};
use motorknob::{MotorKnob, Profile};
use serde::Deserialize;
use std::{fs, path::Path};
use std::process::exit;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Config {
    profiles: Vec<Profile>
}

fn main() -> anyhow::Result<()> {
    let config = load_config()?;
    let mut motor_knob = MotorKnob::new()?;
    let mut current_profile_index = 0;
    let mut current_profile = &config.profiles[current_profile_index];

    println!("Setting firsts Profile");
    motor_knob.write_profile(current_profile)?;

    // libgpiod setup
    let chip = Chip::new("gpiochip0")?;
    let opts = Options::input([26])
        .bias(Bias::PullUp)
        .edge(EdgeDetect::Both)
        .consumer("change-profile-input");
    let mut inputs = chip.request_lines(opts)?;

    let mut last_edge = Edge::Rising;
    let mut next_activation_allowed = Duration::from_millis(0);
    loop {
        let event = inputs.read_event()?;

        // Some simple debounce logic
        // seems to work quite ok
        if event.edge == Edge::Rising {
            next_activation_allowed = event.time + Duration::from_millis(100);
            // println!("Next in {:?}", next_activation_allowed);
        }
        if last_edge == event.edge {
            // println!("Skipping same edge");
            continue;
        }
        last_edge = event.edge;
        if event.time < next_activation_allowed {
            // println!("Skipping");
            continue;
        }

        // Profile Switching
        current_profile_index = (current_profile_index + 1) % config.profiles.len();
        current_profile = &config.profiles[current_profile_index];
        println!("Current Position {}", motor_knob.read_position()?);
        println!("Swtichting Profile to {}", current_profile.name);
        motor_knob.write_profile(&current_profile)?;
        println!("Loaded Profile {:?}", motor_knob.read_current_profile()?);
    }
}

static CONFIG_PATH: &str = "./Config.toml";

/**
 * Loads Config.toml or Defaults
 */
fn load_config() -> anyhow::Result<Config> {
    let default_profile = Profile{
        name: "Default".into(),
        start_position: 60,
        end_position: 120,
        detents: 60,
    };

    if !Path::new(CONFIG_PATH).exists() {
        println!("No config found. Consider creating Config.toml");
        println!("Using default config");
        return Ok(Config {
            profiles: vec![default_profile]
        })
    }

    let contents = match fs::read_to_string(CONFIG_PATH) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not open Config.toml!");
            exit(1);
        }
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load config from Config.toml! Check spelling. Make sure there is atleast one Profile");
            exit(1);
        }
    };

    Ok(config)
}
