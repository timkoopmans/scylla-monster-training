use crate::SPEED;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use rand::Rng;
use ratatui::crossterm::style::Stylize;
use snailshell::{snailprint_d};
use std::thread::sleep;
use std::time::Duration;

pub fn animate() {
    loop {
        redraw();
        sleep(Duration::from_millis(250));
    }
}

pub fn draw() {
    snailprint_d(format!(r#"


╭⌜⎺⎺⎺⎺⌝╮
│  ({}) │
│  {} │
╰╰╰╰╯╯╯╯
    "#, eye(), mouth()).cyan().bold(), 0.0);
}

fn mouth() -> &'static str {
    let mouths = [
        "──⎚",
        "⎚──",
        "⎖──",
        "└⎖─",
        "└⎎─",
        "└⌴─",
        "─⌴─",
    ];

    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..mouths.len());
    mouths[n]
}

fn eye() -> &'static str {
    let eyes = [
        "◉",
        "◎",
        "⦿",
        "⎚",
        "⍬",
        "⏣",
    ];

    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..eyes.len());
    eyes[n]
}

pub fn ask(prompt: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact()
        .unwrap()
}

pub fn say(text: &str) {
    snailprint_d(text, *SPEED);
}

pub fn info(text: &str) {
    snailprint_d(text.cyan(), *SPEED);
}

pub fn warn(text: &str) {
    snailprint_d(text.yellow(), *SPEED);
}

pub fn pass(text: &str) {
    snailprint_d(text.green(), *SPEED);
}

pub fn fail(text: &str) {
    snailprint_d(text.red(), *SPEED);
}

pub fn exit(text: &str) {
    snailprint_d(trim(r#"
    If you need help at any time you can always take a peek at the detailed instructions.
    When you've completed this challenge, just call me again like this:
    "#), *SPEED);

    info(text);
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn redraw() {
    clear();
    draw();
}

fn trim(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim_start())
        .collect::<Vec<&str>>()
        .join("\n")
}
