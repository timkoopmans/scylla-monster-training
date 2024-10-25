use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub description: String,
    pub setup: String,
    pub solve: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub challenges: Vec<Challenge>,
}