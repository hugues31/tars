/// Round the 'array' passed using 'decimals'
pub fn round_array(array: &mut [f64], decimals: u8) {
    let divider = (10.0 as f64).powi(decimals as i32);
    for number in array {
        *number = (*number * divider).round() / divider;
    }
}
