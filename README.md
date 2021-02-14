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

*Known issues:* Arbitrary precision isn't enforced, output for each account will display with the precision of the most precisely measured transaction from the input file.

Account totals can be inaccurate if you dispute a withdrawal and leave it unresolved. This is due to a design mistake that I made.
I chose to derive "available" from "total" and "held" amounts but really I should have made it another piece of state that can be changed independently.
Everything else is correct and the amounts settle correctly in the output if you resolve or chargeback a dispute.
