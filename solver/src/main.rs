use fraction::{Fraction, GenericFraction};
use solver::{
    bernoulli, continued_fraction, moivre_laplace,
};
use tracing::Level;
use wasm_bindgen::JsValue;

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
    // log(format!(
    //     "{}",
    //     bernoulli(
    //         500,
    //         400,
    //         (80, 100)
    //     )
    // ));
    log(format!("{}", moivre_laplace(50, 10, (80, 100), 100)));
}
