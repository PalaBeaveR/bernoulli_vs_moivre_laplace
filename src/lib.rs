use gloo_worker::Worker;
use serde::{Deserialize, Serialize};
use solver::{bernoulli, SolverResult, moivre_laplace};

pub struct BernoulliSolver;
pub struct MoivreLaplaceSolver;

#[derive(Serialize, Deserialize, Clone)]
pub struct SolverRequest {
    pub total: u32,
    pub required: u32,
    pub odds: (u32, u32),
}

impl Worker for BernoulliSolver {
    type Message = ();

    type Input = SolverRequest;

    type Output = SolverResult;

    fn create(
        scope: &gloo_worker::WorkerScope<Self>,
    ) -> Self {
        Self {}
    }

    fn update(
        &mut self,
        scope: &gloo_worker::WorkerScope<Self>,
        msg: Self::Message,
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
        scope: &gloo_worker::WorkerScope<Self>,
    ) -> Self {
        Self {}
    }

    fn update(
        &mut self,
        scope: &gloo_worker::WorkerScope<Self>,
        msg: Self::Message,
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
            moivre_laplace(msg.total, msg.required, msg.odds),
        )
    }
}
