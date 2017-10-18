import numpy
import talib

decimals = 4



def convert_array_to_vec(array):
    array = array[numpy.logical_not(numpy.isnan(array))].round(4)
    string = "["
    for elem in array:
        string += str(elem) + ", "
    string = string[:-2]
    string += "];"
    return string

# Data series
open = numpy.array([1984.03, 1959.83, 2041.42, 2019.04, 1969.53, 2082.75, 2209.52, 2200.9, 2364.04,
                    2543.32, 2423.95, 2483.28, 2604.88, 2393.81, 2231.27, 2420.82, 2544.0, 2766.67, 2919.62, 2763.25])
high = numpy.array([2174.72, 2129.49, 2158.92, 2050.2, 2042.12, 2151.19, 2220.64, 2352.98, 2456.25,
                    2691.53, 2572.81, 2494.14, 2845.93, 2682.66, 2527.13, 2455.68, 2607.54, 2872.17, 3004.26, 3036.05])
low = numpy.array([1934.7, 1921.02, 1793.77, 1887.36, 1919.72, 1868.23, 1991.19, 2011.08, 2193.91,
                   2183.96, 2223.15, 2363.19, 2240.03, 2208.31, 2192.15, 2199.02, 2311.16, 2463.15, 2651.8, 2749.42])
close = numpy.array([1959.83, 2041.42, 2019.04, 1969.53, 2082.75, 2209.52, 2200.9, 2364.04, 2543.32,
                     2423.95, 2483.28, 2604.88, 2393.81, 2231.27, 2420.82, 2544.0, 2766.67, 2919.62, 2763.25, 2922.14])

# Computation
sma = talib.SMA(close, timeperiod=4)
ema = talib.EMA(close, timeperiod=4)
sar = talib.SAR(high, low, acceleration=0.02, maximum=0.2)
rsi = talib.RSI(close, timeperiod = 6)

# Printing
print("let expected = &" + convert_array_to_vec(rsi))
