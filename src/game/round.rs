use rand::prelude::*;

pub(crate) struct Round {
    player: Option<Choice>,
    computer: Option<Choice>,
    result: Option<Result>,
}

impl Round {
    pub(crate) fn new() -> Round {
        Round {
            player: None,
            computer: None,
            result: None,
        }
    }

    pub(crate) fn play(&mut self) -> &Result  {
        let player = get_user_input();
        let computer = get_computer_choice();
        let result = compare_choices(&player, &computer);
        
        println!("You chose: {}", player.to_string());
        println!("Computer chose: {}", computer.to_string());
        println!("You {}", &result.to_string());
        self.player = Some(player);
        self.computer = Some(computer);
        self.result = Some(result);
        self.result.as_ref().unwrap()
    }

   
}
#[derive(Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn to_string(&self) -> &str {
        match self {
            Choice::Rock => "Rock",
            Choice::Paper => "Paper",
            Choice::Scissors => "Scissors",
        }
    }
}

// #[derive(Copy, Clone)]
pub(crate) enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
  fn to_string(&self) -> &str {
     match self {
        Result::Draw => "Draw",
        Result::Lose => "Lose",
        Result::Win => "Win"
     }
  }  
}

fn get_computer_choice() -> Choice {
    let choices = vec![Choice::Rock, Choice::Paper, Choice::Scissors];
    let random_index = rand::thread_rng().gen_range(0..choices.len());
    choices[random_index]
}

fn get_user_input() -> Choice {
    println!("Please choose rock, paper, or scissors: ");
    let mut player_input = String::new();
    std::io::stdin()
        .read_line(&mut player_input)
        .expect("Failed to read line");
    player_input = player_input.trim().to_ascii_lowercase();
    // Cast into choice.
    match player_input.as_str() {
        "rock" => Choice::Rock,
        "paper" => Choice::Paper,
        "scissors" => Choice::Scissors,
        _ => {
            println!("Invalid choice. Try again.");
            get_user_input()
        }
    }
}

fn compare_choices(player: &Choice, computer: &Choice) -> Result {
    match (player, computer) {
        (Choice::Rock, Choice::Rock) => Result::Draw,
        (Choice::Rock, Choice::Paper) => Result::Lose,
        (Choice::Rock, Choice::Scissors) => Result::Win,
        (Choice::Paper, Choice::Rock) => Result::Win,
        (Choice::Paper, Choice::Paper) => Result::Draw,
        (Choice::Paper, Choice::Scissors) => Result::Lose,
        (Choice::Scissors, Choice::Rock) => Result::Lose,
        (Choice::Scissors, Choice::Paper) => Result::Win,
        (Choice::Scissors, Choice::Scissors) => Result::Draw,
    }
}
