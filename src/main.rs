use bernoulli_vs_moivre_laplace::{bernoulli, moivre_laplace};
use fraction::{Fraction, GenericFraction};
use tracing::Level;

fn main() {
    web_sys::console::log_1(&format!("{}", bernoulli(500, 400, GenericFraction::new(95 as u128, 100 as u128))).into());
    web_sys::console::log_1(&format!("{}", moivre_laplace(500, 400, 0.95)).into());
}
