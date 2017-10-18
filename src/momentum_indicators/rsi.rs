use error::Err;

use std::f64::NAN;

/// Relative Strenght Index
pub fn rsi(data: &[f64], period: usize) -> Result<Vec<f64>, Err> {
    if period > data.len() {
        return Err(Err::NotEnoughtData);
    }

    let mut changes = Vec::new();
    for i in 0..data.len() - 1 {
        let change = data[i + 1] - data[i];
        changes.push(change);
    }

    let rsi_range = data.len() - period;

    let mut rsis = vec![NAN ; rsi_range];

    // gains & losses
    let mut gains = Vec::new();
    let mut losses = Vec::new();

    for i in 0..changes.len() {
        if changes[i] > 0.0 {
            gains.push(changes[i]);
            losses.push(0.0);
        } else if changes[i] < 0.0 {
            losses.push(changes[i] * -1.0);
            gains.push(0.0);
        } else {
            gains.push(0.0);
            gains.push(0.0);
        }
    }

    let mut avg_gain: f64 = gains[..period].iter().sum::<f64>() / gains[..period].len() as f64;
    let mut avg_loss: f64 = losses[..period].iter().sum::<f64>() / losses[..period].len() as f64;

    if avg_loss == 0.0 {
        rsis[0] = 100.0;
    } else {
        let rs = avg_gain / avg_loss;
        rsis[0] = 100.0 - (100.0 / (1.0 + rs));
    }

    for i in 1..rsi_range {
        avg_gain = (avg_gain * (period - 1) as f64 + gains[i + (period - 1)]) / period as f64;
        avg_loss = (avg_loss * (period - 1) as f64 + losses[i + (period - 1)]) / period as f64;

        if avg_loss == 0.0 {
            rsis[i] = 100.0;
        } else {
            let rs = avg_gain / avg_loss;
            rsis[i] = 100.0 - (100.0 / (1.0 + rs));
        }
    }

    Ok(rsis)
}
