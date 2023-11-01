use gloo_worker::Worker;
use serde::{Deserialize, Serialize};
use solver::{
    bernoulli, moivre_laplace, moivre_laplace_smart,
    SolverResult,
};

pub struct BernoulliSolver;
pub struct MoivreLaplaceSolver;

#[derive(Serialize, Deserialize, Clone)]
pub struct SolverRequest {
    pub total: u32,
    pub required: u32,
    pub odds: (u32, u32),
    pub iterations: u32,
    pub stable_amount: usize,
    pub precision: usize,
    pub sqrt_iterations: usize
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

#[derive(Serialize, Deserialize, Clone)]
pub enum MoivreLaplaceMode {
    Normal,
    AutomaticIterations,
}

impl Worker for MoivreLaplaceSolver {
    type Message = ();

    type Input = (MoivreLaplaceMode, SolverRequest);

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
        (mode, msg): Self::Input,
        id: gloo_worker::HandlerId,
    ) {
        scope.respond(
            id,
            match mode {
                MoivreLaplaceMode::Normal => {
                    moivre_laplace(
                        msg.total,
                        msg.required,
                        msg.odds,
                        msg.iterations,
                        msg.sqrt_iterations
                    )
                }
                MoivreLaplaceMode::AutomaticIterations => {
                    moivre_laplace_smart(
                        msg.total,
                        msg.required,
                        msg.odds,
                        msg.precision,
                        msg.stable_amount,
                        msg.sqrt_iterations
                    )
                }
            },
        )
    }
}
