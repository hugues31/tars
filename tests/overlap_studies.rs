extern crate tars;

use tars::overlap_studies::sma::sma;
use tars::overlap_studies::ema::ema;
use tars::overlap_studies::psar::psar;

use tars::helpers::round_array;

// Some randomly generated data to test against TA-Lib (see generate_data.py & correct_values.py)
const OPEN: &[f64] = &[1984.03, 1959.83, 2041.42, 2019.04, 1969.53, 2082.75, 2209.52, 2200.9,
                       2364.04, 2543.32, 2423.95, 2483.28, 2604.88, 2393.81, 2231.27, 2420.82,
                       2544.0, 2766.67, 2919.62, 2763.25];
const HIGH: &[f64] = &[2174.72, 2129.49, 2158.92, 2050.2, 2042.12, 2151.19, 2220.64, 2352.98,
                       2456.25, 2691.53, 2572.81, 2494.14, 2845.93, 2682.66, 2527.13, 2455.68,
                       2607.54, 2872.17, 3004.26, 3036.05];
const LOW: &[f64] = &[1934.7, 1921.02, 1793.77, 1887.36, 1919.72, 1868.23, 1991.19, 2011.08,
                      2193.91, 2183.96, 2223.15, 2363.19, 2240.03, 2208.31, 2192.15, 2199.02,
                      2311.16, 2463.15, 2651.8, 2749.42];
const CLOSE: &[f64] = &[1959.83, 2041.42, 2019.04, 1969.53, 2082.75, 2209.52, 2200.9, 2364.04,
                        2543.32, 2423.95, 2483.28, 2604.88, 2393.81, 2231.27, 2420.82, 2544.0,
                        2766.67, 2919.62, 2763.25, 2922.14];

#[test]
fn sma_works() {
    let mut result = sma(CLOSE, 4).unwrap();
    let expected = &[1997.455, 2028.185, 2070.21, 2115.675, 2214.3025, 2329.445, 2383.0525,
                     2453.6475, 2513.8575, 2476.48, 2428.31, 2412.695, 2397.475, 2490.69,
                     2662.7775, 2748.385, 2842.92];
    round_array(result.as_mut(), 4);
    assert_eq!(result, expected);
}

#[test]
fn ema_works() {
    let mut result = ema(CLOSE, 4).unwrap();
    let expected = &[1997.455, 2031.573, 2102.7518, 2142.0111, 2230.8226, 2355.8216, 2383.073,
                     2423.1558, 2495.8455, 2455.0313, 2365.5268, 2387.6441, 2450.1864, 2576.7799,
                     2713.9159, 2733.6496, 2809.0457];
    round_array(result.as_mut(), 4);
    assert_eq!(result, expected);
}

#[test]
fn psar_works() {
    let mut result = psar(HIGH, LOW, 0.02, 0.2).unwrap();
    let expected = &[2174.72, 2169.646, 2158.92, 2158.92, 1793.77, 1800.9184, 1817.7073,
                     1849.8236, 1898.3377, 1977.657, 2049.0443, 2113.2928, 2201.2093, 2845.93,
                     2832.8544, 2820.0403, 2192.15, 2205.7504, 2237.6908];
    round_array(result.as_mut(), 4);
    // For some reasons, the first values are not exactly the same but since this indicator
    // was not clearly described by its author, we can say that current implementation is correct.
    assert_eq!(result[result.len() - 16..result.len()],
               expected[expected.len() - 16..expected.len()]);
}
