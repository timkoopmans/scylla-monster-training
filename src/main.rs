mod challenges;
mod monster;

use crate::monster::animate;
use clap::Parser;
use once_cell::sync::Lazy;

static SPEED: Lazy<f32> = Lazy::new(|| {
    let s = Opt::parse().chat_speed;
    return if s >= 60.0 { s } else { 4096.0 }
});

#[derive(Debug, Parser, Clone)]
struct Opt {
    #[structopt(long, short = 'c', default_value = "000")]
    challenge: String,

    #[structopt(long, short = 's', default_value = "60.0")]
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

