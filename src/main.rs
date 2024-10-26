mod challenges;
mod monster;
mod checks;
mod config;

use crate::config::Config;
use crate::monster::animate;
use clap::Parser;
use once_cell::sync::Lazy;
use std::fs;
use tracing::error;

static SPEED: Lazy<f32> = Lazy::new(|| {
    Opt::parse().animation_speed
});

static CONFIG: Lazy<Config> = Lazy::new(|| {
    serde_yaml::from_str(
        &fs::read_to_string(Opt::parse().config).expect("Failed to parse config")
    ).expect("Failed to parse config")
});

#[derive(Debug, Parser, Clone)]
struct Opt {
    /// Config
    #[structopt(long, short = 'c', default_value = "challenges.yml")]
    config: String,

    /// Challenge number to resume.
    #[structopt(index = 1, default_value = "001")]
    challenge: String,

    /// Solve the challenge
    #[structopt(long, short = 's')]
    solve: bool,

    /// Speed of the chat, in seconds duration. 0.0 to disable.
    #[structopt(long, short = 'a', default_value = "0.2")]
    animation_speed: f32,
}

fn main() {
    let opt = Opt::parse();

    match opt.challenge.as_str() {
        "dance" => {
            animate();
        }
        _ => {
            if let Some(challenge) = CONFIG.challenges.iter().find(|c| c.id == opt.challenge) {
                if opt.solve {
                    challenges::solve(challenge);
                } else {
                    challenges::setup(challenge);
                }
            } else {
                error!("Challenge not found");
            }
        }
    }
}