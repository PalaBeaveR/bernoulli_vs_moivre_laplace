use gloo_worker::Registrable;
use bernoulli_vs_moivre_laplace::{BernoulliSolver};

fn main() {
    BernoulliSolver::registrar().register();
}
