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

use std::fmt::{Display, Write};
#[cfg(not(target_family = "wasm"))]
pub fn log<T: Display>(value: T) {
    println!("{}", value);
}

fn main() {
    let n = 100;
    let p = 8;

    let mut csv = String::new();
    writeln!(csv, "K,MV,B").unwrap();

    for k in 1..n {
        writeln!(csv, "{k},{:.100},{:.100}", moivre_laplace_smart(n, k, (p, 10), 100, 15, 12).probability, bernoulli(n, k, (p, 10)).probability).unwrap();
        println!("k = {k} DONE");
    }

    std::fs::write("mv.csv", csv).unwrap();
}
