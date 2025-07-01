use std::io::stdin;
use std::fs;

pub fn get_a_number(message: String) -> i32 {
  let mut entry = String::new();
  println!("{}", message);
  stdin().read_line(&mut entry).unwrap();
  let parsed_result = entry.trim().parse::<i32>().unwrap();

  return parsed_result;
}

pub fn get_a_float(message: String) -> f64 {
  let mut entry = String::new();
  println!("{}", message);
  stdin().read_line(&mut entry).unwrap();
  let parsed_result = entry.trim().parse::<f64>().unwrap();

  return parsed_result;
}

pub fn get_a_string(message: String) -> String {
  let mut entry = String::new();
  println!("{}", message);
  stdin().read_line(&mut entry).unwrap();
  
  return entry;
}

pub fn get_tasks_data() -> Option<String> {
  if !fs::exists("./src/tasks.txt").expect("Error while fileExists.") {
    return None
  }

  let data = fs::read_to_string("./src/tasks.txt").expect("Error while reading file.");
  let cleaned_data = data.trim().to_string();

  return Some(cleaned_data);
}
