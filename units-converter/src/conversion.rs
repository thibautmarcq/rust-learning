pub fn celsius_vers_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

pub fn fahrenheit_vers_celsius(c: f64) -> f64 {
    (c - 32.0) * 5.0 / 9.0
}
