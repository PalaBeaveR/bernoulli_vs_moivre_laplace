use num_bigint::BigUint;

use fraction::{BigFraction, GenericFraction};

use num_integer::Integer;
use num_rational::Ratio;
use serde::{Deserialize, Serialize};
use time::Duration;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct SolverResult {
    pub probability: GenericFraction<BigUint>,
    pub took: Duration,
    pub iterations: u32,
}

use web_time::Instant;

pub fn bernoulli(
    experiments: u32,
    positive_outcomes: u32,
    positive_probability: FR,
) -> SolverResult {
    let now = Instant::now();

    let (positive_numer, prob_denom) =
        positive_probability.into();

    let negative_numer = &prob_denom - &positive_numer;

    // Picking the biggest of the two factorials in the denumenator of combinations
    let experiment_diff = (experiments - positive_outcomes)
        .max(positive_outcomes);

    // Simplifying the factorial in the numerator of combinations
    let combinations_numer = (experiment_diff + 1
        ..=experiments)
        .map(BigUint::from)
        .product::<BigUint>();

    // Computing the remaining factorial in the denominator
    let combinations_denom = (1..=(experiments
        - experiment_diff))
        .map(BigUint::from)
        .product::<BigUint>();

    let negative_pow = experiments - positive_outcomes;

    // Multiplying combinations, p^k, q^n-k together
    let probability = Ratio::new_raw(
        combinations_numer
            * positive_numer.pow(positive_outcomes)
            * negative_numer.pow(negative_pow),
        combinations_denom
            * prob_denom.pow(positive_outcomes)
            * prob_denom.pow(negative_pow),
    );

    let elapsed = now.elapsed();

    SolverResult {
        probability: GenericFraction::Rational(
            fraction::Sign::Plus,
            probability,
        ),
        took: Duration::microseconds(
            elapsed.as_micros().try_into().unwrap(),
        ),
        iterations: 0,
    }
}

type FR = Ratio<BigUint>;

// fraction manipulations were done by hand for optimization reasons since the library likes to
// reduce the fraction whenever possible which turned out to slow down the function by a
// substantial margin
pub fn moivre_laplace(
    experiments: u32,
    positive_outcomes: u32,
    positive_probability: FR,
    exponentiation_iterations: usize,
    square_root_iterations: usize,
) -> SolverResult {
    let now = Instant::now();

    let experiments: BigUint = experiments.into();
    let positive_outcomes: BigUint =
        positive_outcomes.into();
    let (positive_numer, prob_denom) =
        positive_probability.into();
    let negative_numer = &prob_denom - &positive_numer;

    let np = experiments * positive_numer;
    let two_npq_numer = 2u32 * &np * negative_numer;
    let two_npq_denom = &prob_denom * &prob_denom;

    // Since we need to subtract np from k, we also need to find the least common denominator and
    // scale numerators accordingly. Since denominator of k is equal to 1 we can just multiply it's
    // numerator by np's denominator and get the appropriate value
    let scaled_positive_outcomes =
        positive_outcomes * &prob_denom;

    // This if condition is here to prevent underflowing as we are working with unsigned numbers
    let exp_numer = if scaled_positive_outcomes >= np {
        scaled_positive_outcomes - np
    } else {
        np - scaled_positive_outcomes
    }
    .pow(2)
        * &two_npq_denom;

    let exp_denom = (&prob_denom).pow(2) * &two_npq_numer;

    let (exp_numer, exp_denom) = exp(
        exp_numer,
        exp_denom,
        exponentiation_iterations,
    );

    // These big u128 numbers are an approximation of pi in fraction form
    let root_numer =
        two_npq_numer * 30_246_273_033_735_921u128;
    let root_denom =
        two_npq_denom * 9_627_687_726_852_338u128;

    // function sqrt returns values in order numer, denom. But since we need 1 over sqrt, we just
    // swap around the numer and denom
    let (left_denom, left_numer) = sqrt(
        root_numer,
        root_denom,
        square_root_iterations,
    );

    let probability = Ratio::new_raw(
        left_numer * exp_denom,
        left_denom * exp_numer,
    );

    let elapsed = now.elapsed();

    SolverResult {
        took: Duration::microseconds(
            elapsed.as_micros().try_into().unwrap(),
        ),
        probability: GenericFraction::Rational(
            fraction::Sign::Plus,
            probability,
        ),
        iterations: 0,
    }
}

pub fn sqrt(
    target_numer: BigUint,
    target_denom: BigUint,
    iterations: usize,
) -> (BigUint, BigUint) {
    let (mut guess_top, mut guess_bot) =
        (target_numer.sqrt(), target_denom.sqrt());
    for _ in 0..iterations {
        let inside_top = &target_numer * &guess_bot;
        let inside_bot = &target_denom * &guess_top;
        (guess_top, guess_bot) = add_ratios_raw_raw(
            guess_top, guess_bot, inside_top, inside_bot,
        );
        if guess_top.is_even() {
            guess_top /= 2u32;
        } else {
            guess_bot *= 2u32
        }
    }
    (guess_top, guess_bot)
}

pub fn exp(
    exponent_numer: BigUint,
    exponent_denom: BigUint,
    iterations: usize,
) -> (BigUint, BigUint) {
    let (mut acc_numer, mut acc_denum) =
        (BigUint::from(0u32), BigUint::from(1u32));
    for iter in 0..iterations {
        (acc_numer, acc_denum) = add_ratios_raw_raw(
            acc_numer,
            acc_denum,
            exponent_numer.pow(iter as u32),
            exponent_denom.pow(iter as u32)
                * factorial_new(iter.into()),
        );
    }

    (acc_numer, acc_denum)
}

pub fn factorial_new(base: BigUint) -> BigUint {
    if base <= BigUint::from(1u32) {
        BigUint::from(1u32)
    } else {
        factorial_new(&base - &BigUint::from(1u32)) * base
    }
}

fn add_ratios_raw_raw(
    mut lhs_numer: BigUint,
    lhs_denom: BigUint,
    mut rhs_numer: BigUint,
    rhs_denom: BigUint,
) -> (BigUint, BigUint) {
    let common_denom = lhs_denom.lcm(&rhs_denom);

    let lhs_multiplier = &common_denom / &lhs_denom;
    let rhs_multiplier = &common_denom / &rhs_denom;

    lhs_numer *= lhs_multiplier;
    rhs_numer *= rhs_multiplier;

    (lhs_numer + rhs_numer, common_denom)
}
