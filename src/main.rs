mod challenges;
mod monster;

use crate::monster::animate;
use clap::{Parser, ArgAction};
use once_cell::sync::Lazy;

static TEXT_SCROLL: Lazy<bool> = Lazy::new(|| {
    Opt::parse().disable_text_scroll
});

static SPEED: Lazy<f32> = Lazy::new(|| {
    let s = Opt::parse().chat_speed;
    return if s >= 60.0 && *TEXT_SCROLL { s } else { 4096.0 }
});

#[derive(Debug, Parser, Clone)]
struct Opt {
    #[structopt(long, short = 'c', default_value = "000")]
    challenge: String,

    #[structopt(long, short = 's', default_value = "60.0")]
    chat_speed: f32,

    #[structopt(long, short = 'd', action=ArgAction::SetFalse)]
    disable_text_scroll: bool,
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

