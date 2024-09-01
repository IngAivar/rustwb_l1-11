use std::collections::HashMap;

fn group_temperatures(temperatures: &[f64]) -> HashMap<std::ops::Range<i32>, Vec<f64>> {
    let mut groups: HashMap<std::ops::Range<i32>, Vec<f64>> = HashMap::new();

    for temp in temperatures {
        let key = (*temp as i32 / 10) * 10..(*temp as i32 / 10 + 1) * 10;
        groups.entry(key).or_insert(Vec::new()).push(*temp);
    }

    groups
}

fn main() {
    let temps = &[-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
    let grouped_temps = group_temperatures(temps);

    for (range, values) in grouped_temps {
        println!("{:?}: {:?}", range, values);
    }
}