use std::collections::HashMap;

use itertools::Itertools;

/// A sequence of temperature fluctuations is given
/// (for example: -25.4, -27.0 13.0, 19.0, 15.5, 24.5, -21.0, 32.5).
/// Combine these values into intervals in increments of 10 degrees.
/// The sequence in the subsets is not important.
/// Example:
/// [-30,-20): {-25.0, -27.0, -21.0},
///  [10, 20): {13.0, 19.0, 15.5},
///  [20, 30): {24.5}, etc.

fn itertools(temperature_fluctuations: Vec<f32>) -> HashMap<i32, Vec<f32>> {
    temperature_fluctuations
        .into_iter()
        .into_group_map_by(|value| (value / 10.0).floor() as i32 * 10)
}

fn general(temperature_fluctuations: Vec<f32>) -> HashMap<i32, Vec<f32>> {
    temperature_fluctuations
        .into_iter()
        .map(|number| ((number / 10.0).floor() as i32 * 10, number))
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_default().push(value);
            acc
        })
}

fn main() {
    let temperature_fluctuations = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];

    println!("{:?}", general(temperature_fluctuations.clone()));
    println!("{:?}", itertools(temperature_fluctuations.clone()));
}
