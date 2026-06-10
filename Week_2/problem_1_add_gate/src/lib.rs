use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    pasta::Fp,
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Selector},
    poly::Rotation,
};

#[derive(Clone, Debug)]
pub struct AddGateConfig {
    a: Column<Advice>,
    b: Column<Advice>,
    c: Column<Advice>,
    selector: Selector,
}

#[derive(Clone, Debug, Default)]
pub struct AddGateCircuit {
    pub rows: [(u64, u64, u64); 5],
}

impl Circuit<Fp> for AddGateCircuit {
    type Config = AddGateConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let a = meta.advice_column();
        let b = meta.advice_column();
        let c = meta.advice_column();
        let selector = meta.selector();

        meta.create_gate("a + b = c", |meta| {
            let enabled = meta.query_selector(selector);
            let a = meta.query_advice(a, Rotation::cur());
            let b = meta.query_advice(b, Rotation::cur());
            let c = meta.query_advice(c, Rotation::cur());

            vec![enabled * (a + b - c)]
        });

        AddGateConfig { a, b, c, selector }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "first five rows",
            |mut region| {
                for (offset, &(a, b, c)) in self.rows.iter().enumerate() {
                    config.selector.enable(&mut region, offset)?;

                    region.assign_advice(|| "a", config.a, offset, || Value::known(Fp::from(a)))?;
                    region.assign_advice(|| "b", config.b, offset, || Value::known(Fp::from(b)))?;
                    region.assign_advice(|| "c", config.c, offset, || Value::known(Fp::from(c)))?;
                }

                Ok(())
            },
        )
    }
}

pub fn example_circuit() -> AddGateCircuit {
    AddGateCircuit {
        rows: [(1, 2, 3), (3, 4, 7), (5, 8, 13), (10, 20, 30), (9, 9, 18)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::dev::MockProver;

    #[test]
    fn accepts_valid_witnesses_in_first_five_rows() {
        let circuit = example_circuit();
        let prover = MockProver::run(4, &circuit, vec![]).unwrap();

        prover.assert_satisfied();
    }

    #[test]
    fn rejects_invalid_witnesses() {
        let mut circuit = example_circuit();
        circuit.rows[2] = (5, 8, 12);

        let prover = MockProver::run(4, &circuit, vec![]).unwrap();

        assert!(prover.verify().is_err());
    }
}
