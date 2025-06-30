use crate::utils;

mod helpers;

pub fn converter() {
  let choice: i32 = utils::get_a_number(String::from("What to you want to convert (type the corresponding number) : \n1 : m -> ft\n2 : ft -> m\n3 : °C -> °F\n4 : °F -> °C"));
  
  let valid_choices = 1..4;
  if !valid_choices.contains(&choice) {
    println!("Wrong choice.");
    return;
  }
  
  let value_to_convert: f64 = utils::get_a_float(String::from("Enter the value that you want to convert :"));

  let result: f64 = match choice {
    1 => helpers::convert_meters_to_feets(value_to_convert),
    2 => helpers::convert_feets_to_meters(value_to_convert),
    3 => helpers::convert_celsius_to_fahr(value_to_convert),
    4 => helpers::convert_fahr_to_celsius(value_to_convert),
    _ => 0.0
  };

  println!("The result is {} !", result);
}
