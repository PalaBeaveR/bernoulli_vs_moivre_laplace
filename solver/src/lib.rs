use std::{
    f64::consts::{E, PI},
    fmt::Display,
    ops::Div,
};

use fraction::{
    BigUint, Decimal, Fraction, GenericDecimal,
    GenericFraction, ToPrimitive,
};

fn pow_fraction(
    fract: GenericFraction<BigUint>,
    power: u32,
) -> GenericFraction<BigUint> {
    GenericFraction::new(
        fract.numer().unwrap().pow(power),
        fract.denom().unwrap().pow(power),
    )
}

use js_sys::WebAssembly::Global;
use serde::{Deserialize, Serialize};
use time::Duration;
use tracing::{debug, debug_span, instrument, span};
use web_sys::{Performance, Window};
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct SolverResult {
    pub probability: GenericFraction<BigUint>,
    pub took: Duration,
}

impl Display for SolverResult {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "probability: {:.10000}; time taken: {}",
            self.probability, self.took
        )
    }
}

use web_time::Instant;

pub fn bernoulli(
    total_tests: u32,             // n
    required_to_pass: u32,        // k
    pass_probability: (u32, u32), // p
) -> SolverResult {
    let pass_probability = GenericFraction::new(
        pass_probability.0,
        pass_probability.1,
    );

    let fail_probability = GenericFraction::new(
        BigUint::from(1 as u32),
        BigUint::from(1 as u32),
    ) - pass_probability.clone(); // q

    let now = Instant::now();

    let amount_difference = total_tests - required_to_pass;
    let combinations = GenericFraction::new(
        (amount_difference + 1..=total_tests)
            .map(BigUint::from)
            .product::<BigUint>(),
        (1..=required_to_pass)
            .map(BigUint::from)
            .product::<BigUint>(),
    );

    let probability = combinations
        * pow_fraction(pass_probability, required_to_pass)
        * pow_fraction(
            fail_probability,
            total_tests - required_to_pass,
        );

    let elapsed = now.elapsed();

    SolverResult {
        probability,
        took: Duration::nanoseconds(
            elapsed.as_micros().try_into().unwrap(),
        ),
    }
}

#[inline]
fn fr(num: u32) -> GenericFraction<BigUint> {
    GenericFraction::from(num)
}

fn fr_sqrt(
    fract: GenericFraction<BigUint>,
) -> GenericFraction<BigUint> {
    let denom = fract.denom().unwrap();
    let numer = fract.numer().unwrap();
    GenericFraction::new(
        (numer * denom).sqrt(),
        denom.clone(),
    )
}

fn fr_flip(
    fract: GenericFraction<BigUint>,
) -> GenericFraction<BigUint> {
    GenericFraction::new(
        fract.denom().unwrap().clone(),
        fract.numer().unwrap().clone(),
    )
}
pub fn moivre_laplace(
    n: u32,        // n
    k: u32,        // k
    p: (u32, u32), // p
) -> SolverResult {
    let n = fr(n);
    let k = fr(k);
    let p = GenericFraction::<BigUint>::new(p.0, p.1);
    let pi = GenericFraction::<BigUint>::new(
        428_224_593_349_304u128,
        136_308_121_570_117u128,
    );
    let e = GenericFraction::<BigUint>::new(
        22_526_049_624_551u128,
        8_286_870_547_680u128,
    );

    let now = Instant::now();

    let q = GenericFraction::from(1) - p.clone(); // q
    let np = n * p.clone();
    let npq = np.clone() * q.clone();

    let k_minus_np_squared =
        pow_fraction(k - np.clone(), 2);
    let two_npq = fr(2) * npq.clone();
    let ml_e = pow_fraction(
        e,
        GenericDecimal::<BigUint, usize>::from_fraction(
            k_minus_np_squared / two_npq,
        )
        .floor()
        .to_u32()
        .unwrap(),
    );

    let probability = fr(1)
        / fr_sqrt(fr(2) * pi * npq.clone())
        * fr_flip(ml_e);

    let elapsed = now.elapsed();

    SolverResult {
        probability,
        took: Duration::nanoseconds(
            elapsed.as_micros().try_into().unwrap(),
        ),
    }
}
