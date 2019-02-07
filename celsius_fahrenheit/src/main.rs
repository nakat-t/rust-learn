fn celsius_to_fahrenheit(c: f64) -> f64 {
    ((9.0/5.0) * c) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (5.0/9.0) * (f - 32.0)
}

fn main() {
    println!("C({}) = F({})", 0.0, celsius_to_fahrenheit(0.0));
    println!("F({}) = C({})", 122.0, fahrenheit_to_celsius(122.0));
}
