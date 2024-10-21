use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub description: String,
    pub setup_commands: Vec<String>,
    pub check_commands: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub challenges: Vec<Challenge>,
}