mod game;
use game::Game;

fn main() {
   let max_rounds = get_max_rounds_from_user();
   Game::new(max_rounds).play();
}

fn get_max_rounds_from_user() -> u32 {
   println!("How many rounds would you like to play? ");
   let mut max_rounds_raw = String::new();
   std::io::stdin()
        .read_line(&mut max_rounds_raw)
        .expect("Failed to read line");
   match max_rounds_raw.trim().parse::<u32>() {
       Ok(n) => { return n; },
       Err(_) => {
         println!("Invalid input, please enter a positive integer.");
         return get_max_rounds_from_user();
       },
   }

}