# Problem 2: Alternating Custom Gates

This project builds a Halo2 circuit with two custom gates:

- `a + b = c`
- `a * b = c`

The circuit uses exactly eight rows.

- Even-numbered rows (`0`, `2`, `4`, `6`) enable the addition gate.
- Odd-numbered rows (`1`, `3`, `5`, `7`) enable the multiplication gate.

## Files

- `src/lib.rs`: circuit definition, sample witness values, and tests.

## How to verify

Run:

```bash
cargo test
```

The tests show that a valid alternating witness passes and an invalid multiplication row fails.