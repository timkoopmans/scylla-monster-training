mod challenges;
mod monster;
mod checks;
mod config;

use crate::config::Config;
use crate::monster::animate;
use clap::Parser;
use once_cell::sync::Lazy;
use std::fs;

static SPEED: Lazy<f32> = Lazy::new(|| {
    Opt::parse().chat_speed
});

#[derive(Debug, Parser, Clone)]
struct Opt {
    /// Challenge number to resume. ??'dance' to animate the monster.
    #[structopt(long, short = 'c', default_value = "002")]
    challenge: String,

    /// Speed of the chat, in seconds duration. 0.0 to disable.
    #[structopt(long, short = 's', default_value = "0.0")]
    chat_speed: f32,
}

fn main() {
    let opt = Opt::parse();

    let config: Config = serde_yaml::from_str(
        &fs::read_to_string("challenges.yml").expect("Failed to read challenges.yaml")
    ).expect("Failed to parse YAML");

    match opt.challenge.as_str() {
        "dance" => {
            animate();
        }
        _ => {
            if let Some(challenge) = config.challenges.iter().find(|c| c.id == opt.challenge) {
                challenges::run_challenge(challenge);
            } else {
                eprintln!("Challenge not found");
            }
        }
    }
}