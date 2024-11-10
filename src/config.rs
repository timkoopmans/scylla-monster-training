use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub description: String,
    pub setup: String,
    #[serde(default = "default_solve")]
    pub solve: Vec<String>,
}

fn default_solve() -> Vec<String> {
    Vec::new()
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub challenges: Vec<Challenge>,
}