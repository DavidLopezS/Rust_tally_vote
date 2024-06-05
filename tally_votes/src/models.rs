use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Election {
    pub id: u32,
    pub description: String,
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    pub id: u32,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vote {
    pub contest_id: u32,
    pub choice_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
    pub contest_id: u32,
    pub total_votes: u32,
    pub results: Vec<ChoiceResult>,
    pub winner: ChoiceResult,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChoiceResult {
    pub choice_id: u32,
    pub text: String,
    pub total_count: u32,
}
