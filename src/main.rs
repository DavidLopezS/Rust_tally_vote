mod models;
use models::{Election, Vote, ElectionResult, Choice, ChoiceResult};
use std::{collections::HashMap, error::Error, fs};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let election = read_election("test_files/election.json")?;
    let votes = read_votes("test_files/votes.jsonl")?;

    if !votes.iter().all(|vote| vote.contest_id == election.id){
        return Err(Box::from("Contest ID in votes does not match contest ID in election"))
    }

    let tally = tally_votes(&votes);
    let choice_results = calculate_choice_results(&election.choices, &tally);
    let winner = find_winner(&choice_results).cloned().expect("No winner found");
    let result = ElectionResult{
        contest_id: election.id,
        total_votes: votes.len() as u32,
        results: choice_results,
        winner,
    };
    write_result("result.json", &result)?;
    Ok(())
}


fn read_election(filename: &str) -> Result<Election, Box<dyn Error>> {
    let election_data = fs::read_to_string(filename)?;
    Ok(serde_json::from_str(&election_data)?)
}

fn read_votes(filename: &str) -> Result<Vec<Vote>, Box<dyn Error>> {
    let votes_data = fs::read_to_string(filename)?;
    let votes: Result<Vec<_>, _> =votes_data
        .lines()
        .map(|line| serde_json::from_str(line))
        .collect();
    
    votes.map_err(|err| err.into())
}

fn tally_votes(votes: &[Vote]) -> HashMap<u32, u32> {
    votes.iter().fold(HashMap::new(), |mut tally, vote| {
        *tally.entry(vote.choice_id).or_insert(0) += 1;
        tally
    })
}

fn calculate_choice_results(choices: &[Choice], tally: &HashMap<u32, u32>) -> Vec<ChoiceResult> {
    choices.iter().map(|choice|{
        ChoiceResult {
            choice_id: choice.id,
            text: choice.text.clone(),
            total_count: *tally.get(&choice.id).unwrap_or(&0),
        }
    }).collect()
}

fn find_winner(choice_results: &[ChoiceResult]) -> Option<&ChoiceResult> {
    choice_results.iter().max_by_key(|result| result.total_count)
}

fn write_result(filename: &str, result: &ElectionResult) -> Result<(), Box<dyn Error>> {
    let result_json = serde_json::to_string_pretty(result)?;
    fs::write(filename, result_json)?;
    Ok(())
}
