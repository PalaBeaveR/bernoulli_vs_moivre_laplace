use fraction::{Fraction, GenericFraction};
use solver::{
    bernoulli, moivre_laplace, moivre_laplace_smart
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
    log(format!("{:.100}", moivre_laplace_smart(100, 20, (80, 100), 100, 20).probability));
}
