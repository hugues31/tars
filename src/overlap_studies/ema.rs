use error::Err;

/// Exponential Moving Average
/// Formula :
pub fn ema(data: &[f64], period: usize) -> Result<Vec<f64>, Err> {
    if period > data.len() {
        return Err(Err::NotEnoughtData);
    }
    let mut ema = Vec::new();
    let mut j = 1;

    // get period sma first and calculate the next period period ema
    let sma = (data[0..period]).iter().sum::<f64>() / period as f64;
    let multiplier: f64 = 2.0 / (1.0 + period as f64);
    ema.push(sma);

    // EMA(current) = ( (Price(current) - EMA(prev) ) x Multiplier) + EMA(prev)
    ema.push(((data[period] - sma) * multiplier) + sma);

    // now calculate the rest of the values
    for i in &data[period + 1..data.len()] {
        let tmp = ((*i - ema[j]) * multiplier) + ema[j];
        j = j + 1;
        ema.push(tmp);
    }

    Ok(ema)
}
