use gloo_worker::Registrable;
use bernoulli_vs_moivre_laplace::{BernoulliSolver, MoivreLaplaceSolver};

fn main() {
    MoivreLaplaceSolver::registrar().register();
}
