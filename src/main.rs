mod challenges;
mod monster;

use crate::monster::animate;
use clap::Parser;
use once_cell::sync::Lazy;

static SPEED: Lazy<f32> = Lazy::new(|| {
    Opt::parse().chat_speed
});

#[derive(Debug, Parser, Clone)]
struct Opt {
    /// Challenge number to resume. 🥚'dance' to animate the monster.
    #[structopt(long, short = 'c', default_value = "001")]
    challenge: String,

    /// Speed of the chat, in seconds duration. 0.0 to disable.
    #[structopt(long, short = 's', default_value = "1.0")]
    chat_speed: f32,
}

fn main() {
    let opt = Opt::parse();

    match opt.challenge.as_str() {
        "001" => {
            challenges::c001::solve();
        }
        "002" => {
            challenges::c002::solve();
        }
        "dance" => {
            animate();
        }
        _ => {
            challenges::c000::setup();
        }
    }
}

