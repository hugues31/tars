use error::Err;

/// Simple Moving Average
/// Formula :
pub fn sma(data: &[f64], period: usize) -> Result<Vec<f64>, Err> {
    if period > data.len() {
        return Err(Err::NotEnoughtData);
    }

    let mut result = Vec::new();
    let mut running_total = 0.0;

    // length = data.len() -> i = 0
    //

    for i in 0..data.len() {
        running_total += data[i];
        if i >= period {
            running_total -= data[i - period];
        }
        if i >= period - 1 {
            result.push(running_total / period as f64);
        }
    }
    Ok(result)
}
