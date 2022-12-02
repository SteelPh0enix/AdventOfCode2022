use std::{fs, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GameChoice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for GameChoice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(GameChoice::Rock),
            "B" | "Y" => Ok(GameChoice::Paper),
            "C" | "Z" => Ok(GameChoice::Scissors),
            _ => Err(String::from(s)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GameResult {
    Loss = 0,
    Draw = 3,
    Victory = 6,
}

impl FromStr for GameResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Loss),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Victory),
            _ => Err(String::from(s)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PlayersChoice {
    me: GameChoice,
    opponent: GameChoice,
}

impl PlayersChoice {
    pub fn new(mine: GameChoice, opponents: GameChoice) -> Self {
        PlayersChoice {
            me: mine,
            opponent: opponents,
        }
    }
}

impl FromStr for PlayersChoice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let choices = s.split_whitespace().collect::<Vec<&str>>();
        Ok(PlayersChoice {
            me: GameChoice::from_str(choices[1]).unwrap(),
            opponent: GameChoice::from_str(choices[0]).unwrap(),
        })
    }
}

impl From<PlayersChoice> for GameResult {
    fn from(round: PlayersChoice) -> Self {
        if round.me == round.opponent {
            return GameResult::Draw;
        }

        if (round.me == GameChoice::Rock && round.opponent == GameChoice::Scissors)
            || (round.me == GameChoice::Scissors && round.opponent == GameChoice::Paper)
            || (round.me == GameChoice::Paper && round.opponent == GameChoice::Rock)
        {
            return GameResult::Victory;
        }

        match Self::from(PlayersChoice::new(round.opponent, round.me)) {
            GameResult::Victory => GameResult::Loss,
            GameResult::Loss => GameResult::Victory,
            GameResult::Draw => GameResult::Draw,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RoundPlan {
    choice: GameChoice,
    result: GameResult,
}

impl RoundPlan {
    #[allow(dead_code)]
    fn new(choice: GameChoice, result: GameResult) -> Self {
        RoundPlan { choice, result }
    }
}

impl FromStr for RoundPlan {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let plan = s.split_whitespace().collect::<Vec<&str>>();
        Ok(RoundPlan {
            choice: GameChoice::from_str(plan[0]).unwrap(),
            result: GameResult::from_str(plan[1]).unwrap(),
        })
    }
}

fn predict_my_choice(plan: RoundPlan) -> GameChoice {
    let choices = [GameChoice::Rock, GameChoice::Paper, GameChoice::Scissors];

    *choices
        .iter()
        .filter(|&&choice| GameResult::from(PlayersChoice::new(choice, plan.choice)) == plan.result)
        .next()
        .unwrap()
}

fn calculate_round_score(result: GameResult, choice: GameChoice) -> u32 {
    result as u32 + choice as u32
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();

    let rounds = input
        .lines()
        .map(|line| PlayersChoice::from_str(line).unwrap())
        .collect::<Vec<PlayersChoice>>();

    let results = rounds
        .iter()
        .map(|&round| GameResult::from(round))
        .collect::<Vec<GameResult>>();

    let total_score: u32 = rounds
        .iter()
        .zip(results.iter())
        .map(|(&round, &result)| calculate_round_score(result, round.me))
        .sum();

    println!("Total score is {total_score}");

    let plans = input
        .lines()
        .map(|line| RoundPlan::from_str(line).unwrap())
        .collect::<Vec<RoundPlan>>();

    let predicted_choices = plans
        .iter()
        .map(|&plan| predict_my_choice(plan))
        .collect::<Vec<GameChoice>>();

    let predicted_total_score: u32 = plans
        .iter()
        .zip(predicted_choices.iter())
        .map(|(&plan, &choice)| calculate_round_score(plan.result, choice))
        .sum();

    println!("Predicted total score is {predicted_total_score}");
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        calculate_round_score, predict_my_choice, GameChoice, GameResult, PlayersChoice, RoundPlan,
    };

    #[test]
    fn game_choice_is_correctly_parsed() {
        assert_eq!(GameChoice::from_str("X"), Ok(GameChoice::Rock));
        assert_eq!(GameChoice::from_str("Y"), Ok(GameChoice::Paper));
        assert_eq!(GameChoice::from_str("Z"), Ok(GameChoice::Scissors));

        assert_eq!(GameChoice::from_str("A"), Ok(GameChoice::Rock));
        assert_eq!(GameChoice::from_str("B"), Ok(GameChoice::Paper));
        assert_eq!(GameChoice::from_str("C"), Ok(GameChoice::Scissors));
    }

    #[test]
    fn players_choices_are_correctly_parsed() {
        let test_inputs = [
            "A X", "B X", "C X", "A Y", "B Y", "C Y", "A Z", "B Z", "C Z",
        ];

        let expected_outputs = [
            PlayersChoice::new(GameChoice::Rock, GameChoice::Rock),
            PlayersChoice::new(GameChoice::Rock, GameChoice::Paper),
            PlayersChoice::new(GameChoice::Rock, GameChoice::Scissors),
            PlayersChoice::new(GameChoice::Paper, GameChoice::Rock),
            PlayersChoice::new(GameChoice::Paper, GameChoice::Paper),
            PlayersChoice::new(GameChoice::Paper, GameChoice::Scissors),
            PlayersChoice::new(GameChoice::Scissors, GameChoice::Rock),
            PlayersChoice::new(GameChoice::Scissors, GameChoice::Paper),
            PlayersChoice::new(GameChoice::Scissors, GameChoice::Scissors),
        ];

        test_inputs
            .iter()
            .zip(expected_outputs.iter())
            .for_each(|(&input, &expected)| {
                assert_eq!(PlayersChoice::from_str(input), Ok(expected))
            });
    }

    #[test]
    fn game_rules_are_correct() {
        let test_inputs = [
            PlayersChoice::new(GameChoice::Rock, GameChoice::Rock),
            PlayersChoice::new(GameChoice::Paper, GameChoice::Paper),
            PlayersChoice::new(GameChoice::Scissors, GameChoice::Scissors),
            PlayersChoice::new(GameChoice::Rock, GameChoice::Paper),
            PlayersChoice::new(GameChoice::Rock, GameChoice::Scissors),
            PlayersChoice::new(GameChoice::Paper, GameChoice::Rock),
            PlayersChoice::new(GameChoice::Paper, GameChoice::Scissors),
            PlayersChoice::new(GameChoice::Scissors, GameChoice::Rock),
            PlayersChoice::new(GameChoice::Scissors, GameChoice::Paper),
        ];

        let expected_results = [
            GameResult::Draw,
            GameResult::Draw,
            GameResult::Draw,
            GameResult::Loss,
            GameResult::Victory,
            GameResult::Victory,
            GameResult::Loss,
            GameResult::Loss,
            GameResult::Victory,
        ];

        test_inputs
            .iter()
            .zip(expected_results.iter())
            .for_each(|(&input, &result)| assert_eq!(GameResult::from(input), result));
    }

    #[test]
    fn scores_are_correctly_calculated() {
        let test_inputs = [
            PlayersChoice::new(GameChoice::Rock, GameChoice::Rock),
            PlayersChoice::new(GameChoice::Paper, GameChoice::Paper),
            PlayersChoice::new(GameChoice::Scissors, GameChoice::Scissors),
            PlayersChoice::new(GameChoice::Rock, GameChoice::Paper),
            PlayersChoice::new(GameChoice::Rock, GameChoice::Scissors),
            PlayersChoice::new(GameChoice::Paper, GameChoice::Rock),
            PlayersChoice::new(GameChoice::Paper, GameChoice::Scissors),
            PlayersChoice::new(GameChoice::Scissors, GameChoice::Rock),
            PlayersChoice::new(GameChoice::Scissors, GameChoice::Paper),
        ];

        let test_game_results = test_inputs
            .iter()
            .map(|&input| GameResult::from(input))
            .collect::<Vec<GameResult>>();

        let expected_scores: [u32; 9] = [
            1 + 3,
            2 + 3,
            3 + 3,
            1 + 0,
            1 + 6,
            2 + 6,
            2 + 0,
            3 + 0,
            3 + 6,
        ];

        test_inputs
            .iter()
            .zip(test_game_results.iter())
            .zip(expected_scores.iter())
            .for_each(|((&input, &result), &score)| {
                assert_eq!(calculate_round_score(result, input.me), score)
            })
    }

    #[test]
    fn round_plans_are_correctly_parsed() {
        let test_inputs = [
            "A X", "B X", "C X", "A Y", "B Y", "C Y", "A Z", "B Z", "C Z",
        ];

        let expected_plans = [
            RoundPlan::new(GameChoice::Rock, GameResult::Loss),
            RoundPlan::new(GameChoice::Paper, GameResult::Loss),
            RoundPlan::new(GameChoice::Scissors, GameResult::Loss),
            RoundPlan::new(GameChoice::Rock, GameResult::Draw),
            RoundPlan::new(GameChoice::Paper, GameResult::Draw),
            RoundPlan::new(GameChoice::Scissors, GameResult::Draw),
            RoundPlan::new(GameChoice::Rock, GameResult::Victory),
            RoundPlan::new(GameChoice::Paper, GameResult::Victory),
            RoundPlan::new(GameChoice::Scissors, GameResult::Victory),
        ];

        test_inputs
            .iter()
            .zip(expected_plans.iter())
            .for_each(|(&input, &plan)| assert_eq!(RoundPlan::from_str(input), Ok(plan)))
    }

    #[test]
    fn my_choices_are_correctly_predicted() {
        let test_plans = [
            RoundPlan::new(GameChoice::Rock, GameResult::Loss),
            RoundPlan::new(GameChoice::Paper, GameResult::Loss),
            RoundPlan::new(GameChoice::Scissors, GameResult::Loss),
            RoundPlan::new(GameChoice::Rock, GameResult::Draw),
            RoundPlan::new(GameChoice::Paper, GameResult::Draw),
            RoundPlan::new(GameChoice::Scissors, GameResult::Draw),
            RoundPlan::new(GameChoice::Rock, GameResult::Victory),
            RoundPlan::new(GameChoice::Paper, GameResult::Victory),
            RoundPlan::new(GameChoice::Scissors, GameResult::Victory),
        ];

        let expected_choices = [
            GameChoice::Scissors,
            GameChoice::Rock,
            GameChoice::Paper,
            GameChoice::Rock,
            GameChoice::Paper,
            GameChoice::Scissors,
            GameChoice::Paper,
            GameChoice::Scissors,
            GameChoice::Rock,
        ];

        test_plans
            .iter()
            .zip(expected_choices.iter())
            .for_each(|(&plan, &choice)| assert_eq!(predict_my_choice(plan), choice))
    }
}
