use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    pasta::Fp,
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Selector},
    poly::Rotation,
};

#[derive(Clone, Debug)]
pub struct HighDegreeConfig {
    a: Column<Advice>,
    b: Column<Advice>,
    c: Column<Advice>,
    d: Column<Advice>,
    e: Column<Advice>,
    f: Column<Advice>,
    g: Column<Advice>,
    h: Column<Advice>,
    selector: Selector,
}

#[derive(Clone, Debug)]
pub struct HighDegreeCircuit<const ACTIVE_ROWS: usize> {
    pub rows: [[u64; 8]; ACTIVE_ROWS],
}

impl<const ACTIVE_ROWS: usize> Default for HighDegreeCircuit<ACTIVE_ROWS> {
    fn default() -> Self {
        Self {
            rows: [[0; 8]; ACTIVE_ROWS],
        }
    }
}

impl<const ACTIVE_ROWS: usize> Circuit<Fp> for HighDegreeCircuit<ACTIVE_ROWS> {
    type Config = HighDegreeConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let a = meta.advice_column();
        let b = meta.advice_column();
        let c = meta.advice_column();
        let d = meta.advice_column();
        let e = meta.advice_column();
        let f = meta.advice_column();
        let g = meta.advice_column();
        let h = meta.advice_column();
        let selector = meta.selector();

        meta.create_gate("a * b * c * d * e * f * g = h", |meta| {
            let enabled = meta.query_selector(selector);
            let a = meta.query_advice(a, Rotation::cur());
            let b = meta.query_advice(b, Rotation::cur());
            let c = meta.query_advice(c, Rotation::cur());
            let d = meta.query_advice(d, Rotation::cur());
            let e = meta.query_advice(e, Rotation::cur());
            let f = meta.query_advice(f, Rotation::cur());
            let g = meta.query_advice(g, Rotation::cur());
            let h = meta.query_advice(h, Rotation::cur());

            vec![enabled * (a * b * c * d * e * f * g - h)]
        });

        HighDegreeConfig {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            selector,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "high-degree rows",
            |mut region| {
                for (offset, row) in self.rows.iter().enumerate() {
                    config.selector.enable(&mut region, offset)?;

                    region.assign_advice(|| "a", config.a, offset, || Value::known(Fp::from(row[0])))?;
                    region.assign_advice(|| "b", config.b, offset, || Value::known(Fp::from(row[1])))?;
                    region.assign_advice(|| "c", config.c, offset, || Value::known(Fp::from(row[2])))?;
                    region.assign_advice(|| "d", config.d, offset, || Value::known(Fp::from(row[3])))?;
                    region.assign_advice(|| "e", config.e, offset, || Value::known(Fp::from(row[4])))?;
                    region.assign_advice(|| "f", config.f, offset, || Value::known(Fp::from(row[5])))?;
                    region.assign_advice(|| "g", config.g, offset, || Value::known(Fp::from(row[6])))?;
                    region.assign_advice(|| "h", config.h, offset, || Value::known(Fp::from(row[7])))?;
                }

                Ok(())
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct HighDegreeInstanceConfig {
    a: Column<Advice>,
    b: Column<Advice>,
    c: Column<Advice>,
    d: Column<Advice>,
    e: Column<Advice>,
    f: Column<Advice>,
    g: Column<Advice>,
    selector: Selector,
}

#[derive(Clone, Debug)]
pub struct HighDegreeInstanceCircuit<const ACTIVE_ROWS: usize> {
    pub factors: [[u64; 7]; ACTIVE_ROWS],
}

impl<const ACTIVE_ROWS: usize> Default for HighDegreeInstanceCircuit<ACTIVE_ROWS> {
    fn default() -> Self {
        Self {
            factors: [[0; 7]; ACTIVE_ROWS],
        }
    }
}

impl<const ACTIVE_ROWS: usize> Circuit<Fp> for HighDegreeInstanceCircuit<ACTIVE_ROWS> {
    type Config = HighDegreeInstanceConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let a = meta.advice_column();
        let b = meta.advice_column();
        let c = meta.advice_column();
        let d = meta.advice_column();
        let e = meta.advice_column();
        let f = meta.advice_column();
        let g = meta.advice_column();
        let h = meta.instance_column();
        let selector = meta.selector();

        meta.create_gate("a * b * c * d * e * f * g = h (instance)", |meta| {
            let enabled = meta.query_selector(selector);
            let a = meta.query_advice(a, Rotation::cur());
            let b = meta.query_advice(b, Rotation::cur());
            let c = meta.query_advice(c, Rotation::cur());
            let d = meta.query_advice(d, Rotation::cur());
            let e = meta.query_advice(e, Rotation::cur());
            let f = meta.query_advice(f, Rotation::cur());
            let g = meta.query_advice(g, Rotation::cur());
            let h = meta.query_instance(h, Rotation::cur());

            vec![enabled * (a * b * c * d * e * f * g - h)]
        });

        meta.enable_equality(h);

        HighDegreeInstanceConfig {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            selector,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "high-degree rows with instance output",
            |mut region| {
                for (offset, row) in self.factors.iter().enumerate() {
                    config.selector.enable(&mut region, offset)?;

                    region.assign_advice(|| "a", config.a, offset, || Value::known(Fp::from(row[0])))?;
                    region.assign_advice(|| "b", config.b, offset, || Value::known(Fp::from(row[1])))?;
                    region.assign_advice(|| "c", config.c, offset, || Value::known(Fp::from(row[2])))?;
                    region.assign_advice(|| "d", config.d, offset, || Value::known(Fp::from(row[3])))?;
                    region.assign_advice(|| "e", config.e, offset, || Value::known(Fp::from(row[4])))?;
                    region.assign_advice(|| "f", config.f, offset, || Value::known(Fp::from(row[5])))?;
                    region.assign_advice(|| "g", config.g, offset, || Value::known(Fp::from(row[6])))?;
                }

                Ok(())
            },
        )
    }
}

pub fn example_rows<const ACTIVE_ROWS: usize>() -> [[u64; 8]; ACTIVE_ROWS] {
    std::array::from_fn(|index| {
        let a = (index as u64 % 3) + 1;
        let b = 2;
        let c = 2;
        let d = 2;
        let e = 1;
        let f = 2;
        let g = 1;
        let h = a * b * c * d * e * f * g;

        [a, b, c, d, e, f, g, h]
    })
}

pub fn example_circuit<const ACTIVE_ROWS: usize>() -> HighDegreeCircuit<ACTIVE_ROWS> {
    HighDegreeCircuit {
        rows: example_rows(),
    }
}

pub fn example_factors<const ACTIVE_ROWS: usize>() -> [[u64; 7]; ACTIVE_ROWS] {
    std::array::from_fn(|index| {
        let a = (index as u64 % 3) + 1;
        [a, 2, 2, 2, 1, 2, 1]
    })
}

pub fn expected_outputs<const ACTIVE_ROWS: usize>(factors: &[[u64; 7]; ACTIVE_ROWS]) -> Vec<Fp> {
    factors
        .iter()
        .map(|row| Fp::from(row.iter().copied().product::<u64>()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::dev::MockProver;

    #[test]
    fn accepts_valid_high_degree_rows() {
        let circuit = example_circuit::<4>();
        let prover = MockProver::run(5, &circuit, vec![]).unwrap();

        prover.assert_satisfied();
    }

    #[test]
    fn rejects_an_invalid_high_degree_output() {
        let mut rows = example_rows::<4>();
        rows[1][7] += 1;

        let circuit = HighDegreeCircuit::<4> { rows };
        let prover = MockProver::run(5, &circuit, vec![]).unwrap();

        assert!(prover.verify().is_err());
    }

    #[test]
    fn accepts_the_instance_column_variant() {
        let factors = example_factors::<3>();
        let public_outputs = expected_outputs(&factors);
        let circuit = HighDegreeInstanceCircuit::<3> { factors };
        let prover = MockProver::run(5, &circuit, vec![public_outputs]).unwrap();

        prover.assert_satisfied();
    }

    #[test]
    fn rejects_wrong_public_outputs() {
        let factors = example_factors::<3>();
        let mut public_outputs = expected_outputs(&factors);
        public_outputs[1] += Fp::from(1);

        let circuit = HighDegreeInstanceCircuit::<3> { factors };
        let prover = MockProver::run(5, &circuit, vec![public_outputs]).unwrap();

        assert!(prover.verify().is_err());
    }
}
