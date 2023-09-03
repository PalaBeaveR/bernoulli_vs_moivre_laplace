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

use time::Duration;
use tracing::{debug, debug_span, instrument, span};
use web_sys::{Performance, Window};

pub struct BernoulliResult {
    probability: GenericFraction<BigUint>,
    took: Duration,
}

impl Display for BernoulliResult {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "probability: {:.1000}; time taken: {}",
            self.probability, self.took
        )
    }
}

pub fn bernoulli(
    total_tests: u32,      // n
    required_to_pass: u32, // k
    pass_probability: GenericFraction<BigUint>, // p
) -> BernoulliResult {
    let performance =
        web_sys::window().unwrap().performance().unwrap();

    let fail_probability = GenericFraction::new(
        BigUint::from(1 as u32),
        BigUint::from(1 as u32),
    ) - pass_probability.clone(); // q

    let now = performance.now();

    let amount_difference = total_tests - required_to_pass;
    let combinations = GenericFraction::new(
        (amount_difference + 1..=total_tests)
            .map(BigUint::from)
            .product::<BigUint>(),
        (1..=required_to_pass)
            .map(BigUint::from)
            .product::<BigUint>()
            * (1..=amount_difference)
                .map(BigUint::from)
                .product::<BigUint>(),
    );

    let probability = combinations
        * pow_fraction(pass_probability, required_to_pass)
        * pow_fraction(
            fail_probability,
            total_tests - required_to_pass,
        );

    let elapsed = performance.now() - now;

    BernoulliResult {
        probability,
        took: Duration::milliseconds(elapsed as i64),
    }
}

pub struct MoivreLaplaceResult {
    pub probability: f64,
    took: Duration,
}


impl Display for MoivreLaplaceResult {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "probability: {}; time taken: {}",
            self.probability, self.took
        )
    }
}

pub fn moivre_laplace(
    n: u32, // n
    k: u32, // k
    p: f64, // p
) -> MoivreLaplaceResult {
    let n = n as f64;
    let k = k as f64;
    let performance =
        web_sys::window().unwrap().performance().unwrap();

    let now = performance.now();
    let q = 1. - p; // q
    let probability = 1. / (2. * PI * n * p * q).sqrt()
        * E.powf(-((k - n * p).powi(2) / (2. * n * p * q)));
    let elapsed = performance.now() - now;

    MoivreLaplaceResult { probability, took: Duration::milliseconds(elapsed as i64) }
}
