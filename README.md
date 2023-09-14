# Nearest Color

See **Nearest Color GUI** to get a better idea of what this project is about.

This version runs through all 8-bit colors to find the nearest named color for each.

The code is (currently) single-threaded and only consumes 0.6MB of RAM, taking less than 10 minutes to compute the result on a Ryzen 7700X. The central piece is written in a MapReduce paradigm,
so it should be trivial to convert to multithreaded using Rayon (theoretically).

The program's result has been included in output.txt. Yes, the most common color is Lime Green.

### Rust Crates

* serde
* csv