pub fn convert_meters_to_feets(meters: f64) -> f64 {
  return meters * 3.28084;
}

pub fn convert_feets_to_meters(feets: f64) -> f64 {
  return feets / 3.28084;
}

pub fn convert_celsius_to_fahr(celsius: f64) -> f64 {
  return celsius * (9.0 / 5.0) + 32.0;
}

pub fn convert_fahr_to_celsius(fahr: f64) -> f64 {
  return fahr * (5.0 / 9.0) - 32.0;
}
