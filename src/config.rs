use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub description: String,
    pub setup: Vec<String>,
    pub solve: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub challenges: Vec<Challenge>,
}