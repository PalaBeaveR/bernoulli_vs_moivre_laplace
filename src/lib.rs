use gloo_worker::Worker;
use num_bigint::BigUint;
use num_rational::Ratio;
use serde::{Deserialize, Serialize};
use solver::{bernoulli, moivre_laplace, SolverResult};

pub struct BernoulliSolver;
pub struct MoivreLaplaceSolver;

#[derive(Serialize, Deserialize, Clone)]
pub struct SolverRequest {
    pub total: u32,
    pub required: u32,
    pub odds: Ratio<BigUint>,
    pub iterations: usize,
    pub stable_amount: usize,
    pub precision: usize,
    pub sqrt_iterations: usize,
}

impl Worker for BernoulliSolver {
    type Message = ();

    type Input = SolverRequest;

    type Output = SolverResult;

    fn create(
        _scope: &gloo_worker::WorkerScope<Self>,
    ) -> Self {
        Self {}
    }

    fn update(
        &mut self,
        _scope: &gloo_worker::WorkerScope<Self>,
        _msg: Self::Message,
    ) {
    }

    fn received(
        &mut self,
        scope: &gloo_worker::WorkerScope<Self>,
        msg: Self::Input,
        id: gloo_worker::HandlerId,
    ) {
        scope.respond(
            id,
            bernoulli(msg.total, msg.required, msg.odds),
        )
    }
}

impl Worker for MoivreLaplaceSolver {
    type Message = ();

    type Input = SolverRequest;

    type Output = SolverResult;

    fn create(
        _scope: &gloo_worker::WorkerScope<Self>,
    ) -> Self {
        Self {}
    }

    fn update(
        &mut self,
        _scope: &gloo_worker::WorkerScope<Self>,
        _msg: Self::Message,
    ) {
    }

    fn received(
        &mut self,
        scope: &gloo_worker::WorkerScope<Self>,
        msg: Self::Input,
        id: gloo_worker::HandlerId,
    ) {
        scope.respond(
            id,
            moivre_laplace(
                msg.total,
                msg.required,
                msg.odds,
                msg.iterations,
                msg.sqrt_iterations,
            ),
        )
    }
}
