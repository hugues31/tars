use std::f64::NAN;

use error::Err;

/// Parabolic SAR
/// iaf : increment acceleration factor / starting acceleration. Usually 0.02
/// maxaf : max acceleration factor. Usually 0.2
/// Hypothesis : assuming long for initial conditions
/// Formula :
pub fn psar(high: &[f64], low: &[f64], iaf: f64, maxaf: f64) -> Result<Vec<f64>, Err> {
    let mut psar = vec![NAN; high.len()];

    if high.len() < 2 {
        return Err(Err::NotEnoughtData);
    };

    let mut long = false;
    if high[0] + low[0] <= high[1] + low[1] {
        long = true;
    }

    let mut sar;
    let mut extreme;

    if long {
        extreme = high[0];
        sar = low[0];
    } else {
        extreme = low[0];
        sar = high[0];
    }

    psar[0] = sar;

    let mut af = iaf;

    for i in 1..high.len() {
        sar = (extreme - sar) * af + sar;

        if long {
            if i >= 2 && (sar > low[i - 2]) {
                sar = low[i - 2]
            };
            if sar > low[i - 1] {
                sar = low[i - 1]
            };

            if af < maxaf && high[i] > extreme {
                af += iaf;
                if af > maxaf {
                    af = maxaf
                };
            }

            if high[i] > extreme {
                extreme = high[i];
            }
        } else {
            if i >= 2 && sar < high[i - 2] {
                sar = high[i - 2]
            };
            if sar < high[i - 1] {
                sar = high[i - 1]
            };

            if af < maxaf && low[i] < extreme {
                af += iaf;
                if af > maxaf {
                    af = maxaf
                };
            }

            if low[i] < extreme {
                extreme = low[i]
            };
        }

        if long && low[i] < sar || !long && high[i] > sar {
            af = iaf;
            sar = extreme;

            long = !long;

            if !long {
                extreme = low[i];
            } else {
                extreme = high[i];
            }
        }

        psar[i] = sar;
    }

    Ok(psar)
}
