use std::collections::HashMap;
use tally_votes::models::{Election, Vote, ChoiceResult, Choice, Result};

fn tally_votes(election: &Election, votes: &[Vote]) -> Result {
    // Tally votes
    let mut tally: HashMap<u32, u32> = HashMap::new();
    for vote in votes {
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
    Result {
        contest_id: election.id,
        total_votes: votes.len() as u32,
        results: choice_results,
        winner,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tally_votes() {
        let election = Election {
            id: 1,
            description: "Best Programming Language".to_string(),
            choices: vec![
                Choice { id: 1, text: "Rust".to_string() },
                Choice { id: 2, text: "Python".to_string() },
                Choice { id: 3, text: "Go".to_string() },
            ],
        };

        let votes = vec![
            Vote { contest_id: 1, choice_id: 1 },
            Vote { contest_id: 1, choice_id: 2 },
            Vote { contest_id: 1, choice_id: 1 },
            Vote { contest_id: 1, choice_id: 3 },
        ];

        let result = tally_votes(&election, &votes);

        assert_eq!(result.winner.choice_id, 1);
        assert_eq!(result.total_votes, 4);
    }
}