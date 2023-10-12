use std::{
    f64::consts::{E, PI},
    fmt::Display,
    ops::Div,
};

use num_bigint::{BigInt, BigUint};

fn pow_fraction(
    fract: GenericFraction<BigUint>,
    power: u32,
) -> GenericFraction<BigUint> {
    let GenericFraction::Rational(_, ratio) = fract else {
        panic!("Fraction is not rational")
    };

    let (numer, denom) = ratio.into();
    GenericFraction::new(numer.pow(power), denom.pow(power))
}

use fraction::GenericFraction;
use js_sys::WebAssembly::Global;
use num_rational::Ratio;
use serde::{Deserialize, Serialize};
use time::Duration;
use tracing::{debug, debug_span, instrument, span};
use web_sys::{Performance, Window};
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct SolverResult {
    pub probability: GenericFraction<BigUint>,
    pub took: Duration,
    pub iterations: u32,
}

use web_time::Instant;

pub fn bernoulli(
    total_tests: u32,                     // n
    required_to_pass: u32,                // k
    (pass_numer, pass_denom): (u32, u32), // p
) -> SolverResult {
    let pass_probability = rat(pass_numer, pass_denom);

    let fail_probability =
        &rat_num(1u32) - &pass_probability; // q

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
        iterations: 0,
    }
}

#[inline]
fn fr<T: Into<BigUint>>(
    num: T,
) -> GenericFraction<BigUint> {
    GenericFraction::from(num.into())
}

fn fr_sqrt(
    fract: GenericFraction<BigUint>,
    iterations: usize,
) -> GenericFraction<BigUint> {
    let mut approx = fract.clone();

    for _ in 0..iterations {
        approx =
            (&approx + (&fract / &approx)) / rat_num(2u32);
    }

    approx
}

fn fr_flip(
    fract: GenericFraction<BigUint>,
) -> GenericFraction<BigUint> {
    let GenericFraction::Rational(_, ratio) = fract else {
        panic!("Fraction is not rational")
    };

    let (numer, denom) = ratio.into();

    GenericFraction::new_raw(denom, numer)
}

#[inline]
fn rat<T: Into<BigUint>>(
    numer: T,
    denom: T,
) -> GenericFraction<BigUint> {
    GenericFraction::new(numer.into(), denom.into())
}

#[inline]
fn rat_raw<T: Into<BigUint>>(
    numer: T,
    denom: T,
) -> GenericFraction<BigUint> {
    GenericFraction::new_raw(numer.into(), denom.into())
}

#[inline]
fn rat_num<T: Into<BigUint>>(
    num: T,
) -> GenericFraction<BigUint> {
    GenericFraction::new_raw(num.into(), Into::into(1u32))
}

fn diff(
    a: &GenericFraction<BigUint>,
    b: &GenericFraction<BigUint>,
) -> GenericFraction<BigUint> {
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
    iterations: u32,
) -> SolverResult {
    let now = Instant::now();
    let n = fr(n);
    let k = fr(k);
    let p = rat(pass_numer, pass_denom);
    let pi = rat_raw(
        30_246_273_033_735_921u128,
        9_627_687_726_852_338u128,
    );

    let q = &rat_num(1u32) - &p; // q
    let np = &n * &p;
    let npq = &np * &q;

    let k_minus_np_squared = pow_fraction(diff(&k, &np), 2);
    let two_npq = &rat_num(2u32) * &npq;
    let ml_e =
        exp(k_minus_np_squared / two_npq, iterations);

    let probability =
        fr_flip(fr_sqrt(&(&rat_num(2u32) * &pi) * &npq, 5))
            * fr_flip(ml_e);

    let elapsed = now.elapsed();

    SolverResult {
        probability,
        took: Duration::microseconds(
            elapsed.as_micros().try_into().unwrap(),
        ),
        iterations
    }
}

pub fn moivre_laplace_smart(
    n: u32,                               // n
    k: u32,                               // k
    (pass_numer, pass_denom): (u32, u32), // p
    precision: usize,
    stable_amount: usize,
) -> SolverResult {
    let now = Instant::now();
    let n = fr(n);
    let k = fr(k);
    let p = rat(pass_numer, pass_denom);
    let pi = rat_raw(
        30_246_273_033_735_921u128,
        9_627_687_726_852_338u128,
    );

    let q = &rat_num(1u32) - &p; // q
    let np = &n * &p;
    let npq = &np * &q;

    let k_minus_np_squared = pow_fraction(diff(&k, &np), 2);
    let two_npq = &rat_num(2u32) * &npq;
    let (iterations, ml_e) = finish_moivre(
        k_minus_np_squared / two_npq,
        precision,
        fr_flip(fr_sqrt(&(&rat_num(2u32) * &pi) * &npq, 5)),
        stable_amount,
    );

    let elapsed = now.elapsed();

    SolverResult {
        probability: ml_e,
        took: Duration::microseconds(
            elapsed.as_micros().try_into().unwrap(),
        ),
        iterations
    }
}

pub fn continued_fraction(
    numbers: &[u32],
) -> GenericFraction<BigUint> {
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
    fract: GenericFraction<BigUint>,
    iterations: u32,
) -> GenericFraction<BigUint> {
    let mut result = rat_num(1u32);
    for i in 1..iterations {
        result += pow_fraction(fract.clone(), i)
            / fr(factorial(i.into()));
    }
    result
}

fn finish_moivre(
    fract: GenericFraction<BigUint>,
    precision: usize,
    first_part: GenericFraction<BigUint>,
    stable_amount: usize,
) -> (u32, GenericFraction<BigUint>) {
    let mut leading_zeroes = 0usize;
    let mut stable_num: String = "0".into();
    let mut prob = rat_num(0u32);

    let mut result = rat_num(1u32);
    for i in 1.. {
        result += pow_fraction(fract.clone(), i)
            / fr(factorial(i.into()));

        prob = &first_part / &result;
        let decimal = format!("{:.precision$}", prob);
        let new_leading_zeroes = decimal
            .chars()
            .position(|ch| ch != '0' && ch != '.')
            .unwrap();

        let num = decimal[leading_zeroes
            ..(leading_zeroes + stable_amount)
                .min(decimal.len())]
            .to_string();

        if new_leading_zeroes == leading_zeroes {
            if num == stable_num {
                return (i+1, prob);
            }
            stable_num = num;
            continue;
        }
        leading_zeroes = new_leading_zeroes;
    }

    (0, prob)
}

// pub fn pi_series(iter: u32) {
//     let mut pi: GenericFraction<BigUint> = rat_num(0u32);
//
//     let mut toggle = true;
//     let k1 = 545140134u32;
//     let k2 = 13591409u32;
//     let k3 = 640320u32;
//     let k4 = 100100025u32;
//     let k5 = 327843840u32;
//     let k6 = 53360u32;
//
// use num_integer::Roots;
//     let s = rat_num(k6 * k3.sqrt());
//
//     for i in (0..iter) {
//         if toggle {
//             pi += rat(fact(6 * i) * (k2 + i*k1), fact(i).pow(3)*fact(3*i)*(8*k4*k5).pow(i));
//         } else {
//             pi -= rat(fact(6 * i) * (k2 + i*k1), fact(i).pow(3)*fact(3*i)*(8*k4*k5).pow(i));
//         }
//         toggle = !toggle;
//
//         println!("{:.100}", s / pi);
//     }
// }

fn fact(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        x => x * fact(n - 1),
    }
}

use fraction::BigFraction;

fn to_fraction(
    ratio: GenericFraction<BigUint>,
) -> BigFraction {
    let GenericFraction::Rational(_, ratio) = ratio else {
        panic!("Fraction is not rational")
    };

    let (numer, denom) = ratio.into();
    BigFraction::new(numer, denom)
}
