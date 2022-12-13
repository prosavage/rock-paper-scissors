mod round;

use round::Round;
pub struct Game {
    rounds: Vec<Round>,
    max_rounds: u32,
    total_wins: u32,
    total_draws: u32,
    total_losses: u32,
}

impl Game {
    pub fn new(max_rounds: u32) -> Game {
        Game {
            rounds: Vec::new(),
            max_rounds,
            total_wins: 0,
            total_draws: 0,
            total_losses: 0,
        }
    }

    fn play_round(&mut self) {
        let mut round = Round::new();
        let result = round.play();
        match result {
            round::Result::Win => self.total_wins += 1,
            round::Result::Lose => self.total_losses += 1,
            round::Result::Draw => self.total_draws += 1,
        }
        self.rounds.push(round);
    }

    pub fn play(&mut self) {
        println!("Starting Rock Paper Scissors Game w/ {} rounds.", &self.max_rounds);
        while self.rounds.len() < self.max_rounds.try_into().unwrap() {
            self.print_separator();
            self.play_round();
            self.print_stats();
        }
    }

    fn print_separator(&self) {
      println!("===============");
    }

    fn print_stats(&self) {
      println!("Stats -> Wins: {} Losses: {} Draws: {}", self.total_wins, self.total_losses, self.total_draws)
    }
}
