use fraction::GenericFraction;
use num_bigint::BigUint;
use num_rational::Ratio;
use solver::{bernoulli, moivre_laplace};
use time::Instant;

#[cfg(target_family = "wasm")]
pub fn log<T: Into<JsValue>>(value: T) {
    web_sys::console::log_1(&value.into());
}

use std::fmt::Display;
#[cfg(not(target_family = "wasm"))]
pub fn log<T: Display>(value: T) {
    println!("{}", value);
}

fn main() {
    println!("{:.50}", bernoulli(100, 80, Ratio::new_raw(8u32.into(), 10u32.into())).probability);
    println!("{:.50}", moivre_laplace(100, 80, Ratio::new_raw(8u32.into(), 10u32.into()), 500, 10).probability);
    
}
