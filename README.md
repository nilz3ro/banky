# banky 🐷

This is a toy app that I made as an exercise in [Rust](https://rust-lang.org).

Run the program with `cargo run -- input-1.csv > output.csv`

This implementation handles

- deposits,
- withdrawals,
- disputes,
- dispute resolution,
- chargebacks

## NOTE:

It will bork if there's whitespace in between csv values.
And it has to have a header row.
I've been using `input-1.csv` to test.

known issue: Arbitrary precision isn't enforced, output for each account will display with the precision of the most precisely measured transaction from the input file.
