use std::collections::HashMap;
use tally_votes::models::{Election, Vote, ChoiceResult, Choice};
use std::error::Error;

fn tally_votes(election: &Election, votes: &[Vote]) -> Result<Vec<ChoiceResult>, Box<dyn Error>> {
    let mut tally: HashMap<u32, u32> = HashMap::new();
    for vote in votes {
        *tally.entry(vote.choice_id).or_insert(0) += 1;
    }

    let choice_results: Vec<ChoiceResult> = election.choices.iter().map(|choice| {
        ChoiceResult {
            choice_id: choice.id,
            text: choice.text.clone(),
            total_count: *tally.get(&choice.id).unwrap_or(&0),
        }
    }).collect();

    Ok(choice_results)
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

        let result = tally_votes(&election, &votes).unwrap();

        assert_eq!(result.iter().find(|&r| r.choice_id == 1).unwrap().total_count, 2);
        assert_eq!(result.iter().find(|&r| r.choice_id == 2).unwrap().total_count, 1);
        assert_eq!(result.iter().find(|&r| r.choice_id == 3).unwrap().total_count, 1);
        assert_eq!(result.len(), 3);
    }
}
