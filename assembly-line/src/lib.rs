// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = match speed {
        0..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    };
    let result: f64 =  success_rate * (speed as u32 * 221) as f64;
    result
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
