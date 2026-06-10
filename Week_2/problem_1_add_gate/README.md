# Problem 1: Single Custom Gate

This project builds a Halo2 circuit with one custom gate:

`a + b = c`

The selector is enabled on the first five rows, so the constraint must hold on rows `0` through `4`.

## Files

- `src/lib.rs`: circuit definition, sample witness values, and tests.

## How to verify

Run:

```bash
cargo test
```

The tests show both cases:

- a valid witness assignment satisfies the circuit;
- an invalid assignment is rejected by `MockProver`.