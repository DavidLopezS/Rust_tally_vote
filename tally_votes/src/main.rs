mod models;
use tally_votes::models::{Election, Vote, Result, ChoiceResult};
use std::collections::HashMap;
use std::fs;
use serde_json;

fn main() {
    // Read election data
    let election_data = fs::read_to_string("election.json").expect("Unable to read file");
    let election: Election = serde_json::from_str(&election_data).expect("JSON was not well-formatted");

    // Read votes data
    let votes_data = fs::read_to_string("votes.jsonl").expect("Unable to read file");
    let votes: Vec<Vote> = votes_data
        .lines()
        .map(|line| serde_json::from_str(line).expect("JSON was not well-formatted"))
        .collect();

    // Tally votes
    let mut tally: HashMap<u32, u32> = HashMap::new();
    for vote in &votes {
        *tally.entry(vote.choice_id).or_insert(0) += 1;
    }

    // Prepare results
    let choice_results: Vec<ChoiceResult> = election.choices.iter().map(|choice| {
        ChoiceResult {
            choice_id: choice.id,
            text: choice.text.clone(),
            total_count: *tally.get(&choice.id).unwrap_or(&0),
        }
    }).collect();

    // Determine winner
    let winner = choice_results.iter().max_by_key(|result| result.total_count).unwrap().clone();

    // Create result
    let result = Result {
        contest_id: election.id,
        total_votes: votes.len() as u32,
        results: choice_results,
        winner,
    };

    // Output result
    let result_json = serde_json::to_string_pretty(&result).expect("JSON serialization failed");
    fs::write("result.json", result_json).expect("Unable to write file");
}
