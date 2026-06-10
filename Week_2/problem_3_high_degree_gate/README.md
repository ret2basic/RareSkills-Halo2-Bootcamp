# Problem 3: High-Degree Custom Gate

This project builds a Halo2 circuit with the custom gate:

`a * b * c * d * e * f * g = h`

The number of constrained rows is a compile-time circuit parameter named `ACTIVE_ROWS`.

That means the selector pattern is fixed when the circuit type is chosen. For example, `HighDegreeCircuit<4>` enables the gate on rows `0` through `3`.

## Why the parameter is called `ACTIVE_ROWS`

Halo2 already uses the name `k` for the circuit size parameter passed to `MockProver::run`, where the circuit has `2^k` rows.

To avoid confusion, this example names the compile-time row-count parameter `ACTIVE_ROWS`.

## Challenge: make `h` a public input

This crate also includes a second circuit, `HighDegreeInstanceCircuit`, where `h` is an instance column instead of an advice column.

The important changes are:

- In `configure`, replace the advice column for `h` with `meta.instance_column()`.
- In the gate, use `meta.query_instance(h, Rotation::cur())`.
- In `synthesize`, stop assigning `h` as advice; only assign `a` through `g`.
- Pass the public values when the prover is created, for example with `MockProver::run(..., vec![public_outputs])`.

That last point is the key answer to the challenge question: instance values are not stored inside the circuit struct. They are supplied separately as public inputs when the prover is run.

## How to verify

Run:

```bash
cargo test
```

The tests cover:

- the valid high-degree advice-column circuit;
- rejection of an invalid private `h` value;
- the valid instance-column challenge variant;
- rejection of an invalid public output.