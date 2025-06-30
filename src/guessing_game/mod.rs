use rand::random_range;
use crate::utils;

pub fn guess_number() {
    let mystery_number = random_range(0..100);

    let mut guessed_number = utils::get_a_number(String::from("Enter a number between 1 and 100 :"));

    while guessed_number != mystery_number {
      match guessed_number > mystery_number {
        true =>  println!("Too big !"),
        false => println!("Too small !")
      }

      guessed_number = utils::get_a_number(String::from("Enter a number between 1 and 100 :"));
    }

    println!("Good job, you found the number {} !!", mystery_number);
}
