use rand::random_range;
use std::{io::stdin};

fn get_a_number(message: String) -> i32 {
  let mut entry = String::new();
  println!("{}", message);
  stdin().read_line(&mut entry).unwrap();
  let parsed_result = entry.trim().parse::<i32>().unwrap();

  return parsed_result;
}

fn get_a_float(message: String) -> f64 {
  let mut entry = String::new();
  println!("{}", message);
  stdin().read_line(&mut entry).unwrap();
  let parsed_result = entry.trim().parse::<f64>().unwrap();

  return parsed_result;
}

fn guess_number() {
    let mystery_number = random_range(0..100);

    let mut guessed_number = get_a_number(String::from("Enter a number between 1 and 100 :"));

    while guessed_number != mystery_number {
        match guessed_number > mystery_number {
            true =>  println!("Too big !"),
            false => println!("Too small !")
        }

        guessed_number = get_a_number(String::from("Enter a number between 1 and 100 :"));
    }

    println!("Good job, you found the number {} !!", mystery_number);
}

fn convert_meters_to_feets(meters: f64) -> f64 {
  return meters * 3.28084;
}

fn convert_feets_to_meters(feets: f64) -> f64 {
  return feets / 3.28084;
}

fn convert_celsius_to_fahr(celsius: f64) -> f64 {
  return celsius * (9.0 / 5.0) + 32.0;
}

fn convert_fahr_to_celsius(fahr: f64) -> f64 {
  return fahr * (5.0 / 9.0) - 32.0;
}

fn converter() {
  let choice: i32 = get_a_number(String::from("What to you want to convert (type the corresponding number) : \n1 : m -> ft\n2 : ft -> m\n3 : °C -> °F\n4 : °F -> °C"));
  
  if choice != 1 && choice != 2 && choice != 3 && choice != 4 {
    println!("Wrong choice.");
    return;
  }
  
  let value_to_convert: f64 = get_a_float(String::from("Enter the value that you want to convert :"));

  let result: f64 = match choice {
    1 => convert_meters_to_feets(value_to_convert),
    2 => convert_feets_to_meters(value_to_convert),
    3 => convert_celsius_to_fahr(value_to_convert),
    4 => convert_fahr_to_celsius(value_to_convert),
    _ => 0.0
  };

  println!("The result is {} !", result);
}

fn main() {
    println!("Hello, world!");
    guess_number();
    converter();
}
