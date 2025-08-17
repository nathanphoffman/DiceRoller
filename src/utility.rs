pub fn round_to_one_place(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}