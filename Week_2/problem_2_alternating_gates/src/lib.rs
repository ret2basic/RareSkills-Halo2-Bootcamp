use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    pasta::Fp,
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Selector},
    poly::Rotation,
};

#[derive(Clone, Debug)]
pub struct AlternatingGateConfig {
    a: Column<Advice>,
    b: Column<Advice>,
    c: Column<Advice>,
    add_selector: Selector,
    mul_selector: Selector,
}

#[derive(Clone, Debug, Default)]
pub struct AlternatingGateCircuit {
    pub rows: [(u64, u64, u64); 8],
}

impl Circuit<Fp> for AlternatingGateCircuit {
    type Config = AlternatingGateConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let a = meta.advice_column();
        let b = meta.advice_column();
        let c = meta.advice_column();
        let add_selector = meta.selector();
        let mul_selector = meta.selector();

        meta.create_gate("a + b = c", |meta| {
            let enabled = meta.query_selector(add_selector);
            let a = meta.query_advice(a, Rotation::cur());
            let b = meta.query_advice(b, Rotation::cur());
            let c = meta.query_advice(c, Rotation::cur());

            vec![enabled * (a + b - c)]
        });

        meta.create_gate("a * b = c", |meta| {
            let enabled = meta.query_selector(mul_selector);
            let a = meta.query_advice(a, Rotation::cur());
            let b = meta.query_advice(b, Rotation::cur());
            let c = meta.query_advice(c, Rotation::cur());

            vec![enabled * (a * b - c)]
        });

        AlternatingGateConfig {
            a,
            b,
            c,
            add_selector,
            mul_selector,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "eight alternating rows",
            |mut region| {
                for (offset, &(a, b, c)) in self.rows.iter().enumerate() {
                    if offset % 2 == 0 {
                        config.add_selector.enable(&mut region, offset)?;
                    } else {
                        config.mul_selector.enable(&mut region, offset)?;
                    }

                    region.assign_advice(|| "a", config.a, offset, || Value::known(Fp::from(a)))?;
                    region.assign_advice(|| "b", config.b, offset, || Value::known(Fp::from(b)))?;
                    region.assign_advice(|| "c", config.c, offset, || Value::known(Fp::from(c)))?;
                }

                Ok(())
            },
        )
    }
}

pub fn example_circuit() -> AlternatingGateCircuit {
    AlternatingGateCircuit {
        rows: [
            (1, 2, 3),
            (2, 3, 6),
            (4, 5, 9),
            (3, 7, 21),
            (10, 11, 21),
            (4, 5, 20),
            (8, 13, 21),
            (6, 7, 42),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::dev::MockProver;

    #[test]
    fn accepts_valid_even_add_and_odd_mul_rows() {
        let circuit = example_circuit();
        let prover = MockProver::run(5, &circuit, vec![]).unwrap();

        prover.assert_satisfied();
    }

    #[test]
    fn rejects_an_invalid_mul_row() {
        let mut circuit = example_circuit();
        circuit.rows[3] = (3, 7, 20);

        let prover = MockProver::run(5, &circuit, vec![]).unwrap();

        assert!(prover.verify().is_err());
    }
}
