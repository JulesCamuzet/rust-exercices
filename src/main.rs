use rand::random_range;
use std::{io::stdin};
use std::fs;
use std::str;

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
  
  let valid_choices = 1..4;
  if !valid_choices.contains(&choice) {
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

fn display_tasks() {
  if !fs::exists("./src/tasks.txt").expect("Error while fileExists.") {
    println!("No tasks file created yet.");
    return
  }

  let data = fs::read_to_string("./src/tasks.txt").expect("Error while reading file.");
  let cleaned_data = data.trim();

  let global_split = str::split(&cleaned_data, "|");
  let mut index = 0;

  for part in global_split {
    if index == 0 {
      println!("TO DO :\n")
    } else if index == 1 {
      println!("\nDONE :\n")
    }

    let tasks_split = part.trim().split('\n');

    for task in tasks_split {
      println!("{}\n", task.trim());
    }

    index += 1;
  }
}

fn tasks_manager() {
  let mut keep_going = true;

  while keep_going {
    let choice: i32 = get_a_number(String::from("What do you want to do : \n1 : Display your tasks\n2 : Mark a task as done\n3 : Mark a task as undone\n4 : Delete a task\n5 : Update a task"));
  
    let valid_choices = 1..5;
    if !valid_choices.contains(&choice) {
      println!("Wrong choice.");
      return;
    }
  
    match choice {
      1 => display_tasks(),
      2 => println!("Coming soon"),
      3 => println!("Coming soon"),
      4 => println!("Coming soon"),
      5 => println!("Coming soon"),
      6 => keep_going = false,
      _ => return
    }
  }
}

fn main() {
    tasks_manager();
}
