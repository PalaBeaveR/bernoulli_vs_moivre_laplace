use std::{
    f64::consts::{E, PI},
    fmt::Display,
    ops::Div,
};

use num_bigint::{BigUint, BigInt};

fn pow_fraction(
    fract: Ratio<BigUint>,
    power: u32,
) -> Ratio<BigUint> {
    let (numer, denom) = fract.into();
    Ratio::new(numer.pow(power), denom.pow(power))
}

use js_sys::WebAssembly::Global;
use num_rational::Ratio;
use serde::{Deserialize, Serialize};
use time::Duration;
use tracing::{debug, debug_span, instrument, span};
use web_sys::{Performance, Window};
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct SolverResult {
    pub probability: Ratio<BigUint>,
    pub took: Duration,
}

use web_time::Instant;

pub fn bernoulli(
    total_tests: u32,                     // n
    required_to_pass: u32,                // k
    (pass_numer, pass_denom): (u32, u32), // p
) -> SolverResult {
    let pass_probability = rat(pass_numer, pass_denom);

    let fail_probability =
        rat_num(1u32) - &pass_probability; // q

    let now = Instant::now();

    let amount_difference = total_tests - required_to_pass;
    let combinations = rat(
        (amount_difference + 1..=total_tests)
            .map(BigUint::from)
            .product::<BigUint>(),
        (1..=required_to_pass)
            .map(BigUint::from)
            .product::<BigUint>(),
    );

    let probability = &combinations
        * pow_fraction(pass_probability, required_to_pass)
        * pow_fraction(
            fail_probability,
            total_tests - required_to_pass,
        );

    let elapsed = now.elapsed();

    SolverResult {
        probability,
        took: Duration::microseconds(
            elapsed.as_micros().try_into().unwrap(),
        ),
    }
}

#[inline]
fn fr<T: Into<BigUint>>(num: T) -> Ratio<BigUint> {
    Ratio::from(num.into())
}

fn fr_sqrt(
    fract: Ratio<BigUint>,
    iterations: usize,
) -> Ratio<BigUint> {
    let mut approx = fract.clone();

    for _ in 0..iterations {
        approx =
            (&approx + (&fract / &approx)) / rat_num(2u32);
    }

    approx
}

fn fr_flip(fract: Ratio<BigUint>) -> Ratio<BigUint> {
    let (numer, denom) = fract.into();

    Ratio::new_raw(denom, numer)
}

#[inline]
fn rat<T: Into<BigUint>>(
    numer: T,
    denom: T,
) -> Ratio<BigUint> {
    Ratio::new(numer.into(), denom.into())
}

#[inline]
fn rat_raw<T: Into<BigUint>>(
    numer: T,
    denom: T,
) -> Ratio<BigUint> {
    Ratio::new_raw(numer.into(), denom.into())
}

#[inline]
fn rat_num<T: Into<BigUint>>(num: T) -> Ratio<BigUint> {
    Ratio::new_raw(num.into(), Into::into(1u32))
}

fn diff(a: &Ratio<BigUint>, b: &Ratio<BigUint>) -> Ratio<BigUint> {
    leptos::log!("a: {} b: {}", a, b);
    if a > b {
        a - b
    } else {
        b - a
    }

}

pub fn moivre_laplace(
    n: u32,                               // n
    k: u32,                               // k
    (pass_numer, pass_denom): (u32, u32), // p
    taylor_iterations: u32,
) -> SolverResult {
    let now = Instant::now();
    let n = fr(n);
    let k = fr(k);
    let p = rat(pass_numer, pass_denom);
    let pi = rat_raw(
        30_246_273_033_735_921u128,
        9_627_687_726_852_338u128,
    );

    let q = rat_num(1u32) - &p; // q
    let np = &n * &p;
    let npq = &np * &q;

    let k_minus_np_squared = pow_fraction(diff(&k, &np), 2);
    let two_npq = rat_num(2u32) * &npq;
    let ml_e = exp(
        k_minus_np_squared / two_npq,
        taylor_iterations,
    );

    let probability = rat_num(1u32)
        / fr_sqrt(rat_num(2u32) * &pi * &npq, 5)
        * fr_flip(ml_e);

    let elapsed = now.elapsed();

    SolverResult {
        probability,
        took: Duration::microseconds(
            elapsed.as_micros().try_into().unwrap(),
        ),
    }
}

pub fn continued_fraction(
    numbers: &[u32],
) -> Ratio<BigUint> {
    match numbers.len() {
        x if x <= 0 => fr(0u32),
        1 => fr(numbers[0]),
        _ => {
            fr(numbers[0])
                + fr_flip(continued_fraction(&numbers[1..]))
        }
    }
}

fn factorial(num: BigUint) -> BigUint {
    let one = BigUint::from(1u32);
    if num <= one {
        one
    } else {
        factorial(&num - one) * &num
    }
}

fn exp(
    fract: Ratio<BigUint>,
    iterations: u32,
) -> Ratio<BigUint> {
    let mut result = rat_num(1u32);
    for i in 1..iterations {
        result += pow_fraction(fract.clone(), i)
            / fr(factorial(i.into()));
    }
    result
}
