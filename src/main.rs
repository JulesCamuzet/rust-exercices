mod utils;
mod guessing_game;
mod tasks;
mod converter;

fn main() {
  let choice = utils::get_a_number(String::from("What do you want to do :\n1 : Convert something\n2 : Play a guessing game\n3 : Manage your tasks"));

  let possible_choices = 1..4;

  if !possible_choices.contains(&choice) {
    println!("Wrong choice.");
    return;
  }

  match choice {
    1 => converter::converter(),
    2 => guessing_game::guess_number(),
    3 => tasks::tasks_manager(),
    _ => return
  }
}
