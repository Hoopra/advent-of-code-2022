#[derive(Debug, Clone)]
pub enum GameMove {
    Rock,
    Paper,
    Scissors,
}

impl GameMove {
    pub fn from_string(input: &str) -> Self {
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" | _ => Self::Scissors,
        }
    }

    pub fn from_string_with_result(opponent_move: &GameMove, input: &str) -> Self {
        match input {
            // lose
            "X" => match opponent_move {
                GameMove::Rock => GameMove::Scissors,
                GameMove::Paper => GameMove::Rock,
                GameMove::Scissors => GameMove::Paper,
            },
            // draw
            "Y" => opponent_move.clone(),
            // win
            "Z" | _ => match opponent_move {
                GameMove::Rock => GameMove::Paper,
                GameMove::Paper => GameMove::Scissors,
                GameMove::Scissors => GameMove::Rock,
            },
        }
    }
}

impl GameMove {
    pub fn score(&self) -> usize {
        match self {
            GameMove::Rock => 1,
            GameMove::Paper => 2,
            GameMove::Scissors => 3,
        }
    }
}

pub struct Game {
    player_move: GameMove,
    opponent_move: GameMove,
}

impl Game {
    pub fn from_string(input: &str) -> Self {
        let mut components = input.split(" ");

        Self {
            opponent_move: GameMove::from_string(components.nth(0).unwrap()),
            player_move: GameMove::from_string(components.nth(0).unwrap()),
        }
    }

    pub fn from_string_with_result(input: &str) -> Self {
        let mut components = input.split(" ");

        let opponent_move = GameMove::from_string(components.nth(0).unwrap());

        Self {
            player_move: GameMove::from_string_with_result(
                &opponent_move,
                components.nth(0).unwrap(),
            ),
            opponent_move,
        }
    }
}

impl Game {
    pub fn score(&self) -> usize {
        self.player_move.score() + self.evaluate()
    }

    pub fn evaluate(&self) -> usize {
        match self.player_move {
            GameMove::Rock => match self.opponent_move {
                GameMove::Rock => 3,
                GameMove::Paper => 0,
                GameMove::Scissors => 6,
            },
            GameMove::Paper => match self.opponent_move {
                GameMove::Rock => 6,
                GameMove::Paper => 3,
                GameMove::Scissors => 0,
            },
            GameMove::Scissors => match self.opponent_move {
                GameMove::Rock => 0,
                GameMove::Paper => 6,
                GameMove::Scissors => 3,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn calculates_game_score() {
        assert_eq!(Game::from_string("A X").score(), 4);
        assert_eq!(Game::from_string("A Y").score(), 8);
        assert_eq!(Game::from_string("A Z").score(), 3);

        assert_eq!(Game::from_string("B X").score(), 1);
        assert_eq!(Game::from_string("B Y").score(), 5);
        assert_eq!(Game::from_string("B Z").score(), 9);

        assert_eq!(Game::from_string("C X").score(), 7);
        assert_eq!(Game::from_string("C Y").score(), 2);
        assert_eq!(Game::from_string("C Z").score(), 6);
    }
}
