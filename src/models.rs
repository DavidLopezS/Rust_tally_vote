use serde::{Deserialize, Serialize};

/// Model of the election with a list of choices.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Election {
    pub id: u32,
    pub description: String,
    pub choices: Vec<Choice>,
}

/// Model of the choice of the election.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    pub id: u32,
    pub text: String,
}

/// Model of the vote in the election.
#[derive(Serialize, Deserialize, Debug)]
pub struct Vote {
    pub contest_id: u32,
    pub choice_id: u32,
}

/// Model of the result of the election.
#[derive(Serialize, Deserialize, Debug)]
pub struct ElectionResult {
    pub contest_id: u32,
    pub total_votes: u32,
    pub results: Vec<ChoiceResult>,
    pub winner: ChoiceResult,
}

/// Model of the result for a particular choice
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChoiceResult {
    pub choice_id: u32,
    pub text: String,
    pub total_count: u32,
}
