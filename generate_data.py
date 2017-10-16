import random

# This script is not perfect but does what I asked

# Options
length = 20  # Length of the dataset
base_price = 1900
volatility = 10  # in percent
rounding = 2  # decimals


open = list()
high = list()
low = list()
close = list()

open.append(random.uniform(base_price * (1 - volatility / 200),
                           base_price * (1 + volatility / 200)))

close.append(random.uniform(base_price * (1 - volatility / 200),
                            base_price * (1 + volatility / 200)))

high.append(max(open[0], close[0]) *
            (1 + random.uniform(0, volatility / 100)))

low.append(min(open[0], close[0]) *
           (1 - random.uniform(0, volatility / 100)))

# Generate dataset
for i in range(1, 20):
    close.append(close[i - 1] * random.uniform(1 -
                                               volatility / 100, 1 + volatility / 100))
    open.append(close[i - 1])
    high.append(max(open[i - 1], close[i - 1]) *
                (1 + random.uniform(0, volatility / 100)))
    low.append(min(open[i - 1], close[i - 1]) *
               (1 - random.uniform(0, volatility / 100)))


# Rounding
for i in range(0, 20):
    open[i] = round(open[i], rounding)
    high[i] = round(high[i], rounding)
    low[i] = round(low[i], rounding)
    close[i] = round(close[i], rounding)

    print("Open : " + str(open[i]))
    print("High : " + str(high[i]))
    print("Low : " + str(low[i]))
    print("Close : " + str(close[i]))
    print("")

# Output for Python
open_str = ""
high_str = ""
low_str = ""
close_str = ""
for i in range(0, 20):
    open_str += str(open[i]) + ", "
    high_str += str(high[i]) + ", "
    low_str += str(low[i]) + ", "
    close_str += str(close[i]) + ", "

print("# Python Output #")
print("open = numpy.array([" + open_str[:-2] + "])")
print("high = numpy.array([" + high_str[:-2] + "])")
print("low = numpy.array([" + low_str[:-2] + "])")
print("close = numpy.array([" + close_str[:-2] + "])")
print("")

# Output for Rust
print("# Rust Output #")
print("const OPEN: &[f64] = &[" + open_str[:-2] + "];")
print("const HIGH: &[f64] = &[" + high_str[:-2] + "];")
print("const LOW: &[f64] = &[" + low_str[:-2] + "];")
print("const CLOSE: &[f64] = &[" + close_str[:-2] + "];")
