use fraction::BigFraction;
use gloo_worker::Spawnable;
use leptos::svg::view;
use leptos::{
    create_effect, event_target, spawn_local, store_value,
    watch, CollectView, RwSignal,
};
use leptos::{create_rw_signal, ReadSignal, Signal};

use fraction::GenericFraction;

use leptos::{
    component, create_signal, event_target_value,
    leptos_dom::console_log, view, IntoView, SignalGet,
    SignalGetUntracked, SignalSet, SignalUpdate,
};
use num_bigint::BigUint;
use num_rational::{BigRational, Ratio};
use serde::{Deserialize, Serialize};
use solver::{bernoulli, SolverResult};

use bernoulli_vs_moivre_laplace::{
    BernoulliSolver, MoivreLaplaceMode,
    MoivreLaplaceSolver, SolverRequest,
};
use web_sys::HtmlInputElement;
use web_time::Instant;

#[component]
pub fn App() -> impl IntoView {
    let (bernoulli_result, set_bernoulli_result) =
        create_signal::<Option<SolverResult>>(None);
    let (bernoulli_running, set_bernoulli_running) =
        create_signal(false);
    let (
        moivre_laplace_running,
        set_moivre_laplace_running,
    ) = create_signal(false);
    let (moivre_laplace_result, set_moivre_laplace_result) =
        create_signal::<Option<SolverResult>>(None);

    let bernoulli_solver = BernoulliSolver::spawner()
        .callback(move |result| {
            set_bernoulli_result(Some(result));
            set_bernoulli_running(false)
        })
        .spawn("./bernoulli.js");
    let bernoulli_solver =
        Box::leak(Box::new(bernoulli_solver));

    let iterations_done = create_rw_signal(0);

    let moivre_laplace_solver =
        MoivreLaplaceSolver::spawner()
            .callback(move |result| {
                iterations_done.set(result.iterations);
                set_moivre_laplace_result(Some(result));
                set_moivre_laplace_running(false);
            })
            .spawn("./moivre_laplace.js");
    let moivre_laplace_solver =
        Box::leak(Box::new(moivre_laplace_solver));

    let variables = Variables {
        total_experiments: 100.into(),
        required_to_pass: 50.into(),
        denominator: 100.into(),
        pass_numerator: 80.into(),
        fail_numerator: 20.into(),
        precision: 1000.into(),
        iterations: 300.into(),
        automatic_iterations: false.into(),
        stable_amount: 5.into(),
        sqrt_iterations: 10.into()
    };

    let np = Signal::derive(move || {
        GenericFraction::<BigUint>::new_raw(
            variables.total_experiments.get().into(),
            1u32.into(),
        ) * BigFraction::new_raw(
            variables.pass_numerator.get().into(),
            variables.denominator.get().into(),
        )
    });
    let npq = Signal::derive(move || {
        np.get()
            * BigFraction::new_raw(
                variables.fail_numerator.get().into(),
                variables.denominator.get().into(),
            )
    });

    let derived_variables = DerivedVariables { np, npq };

    let pass_updated = store_value(false);
    let fail_updated = store_value(false);

    create_effect(move |_| {
        leptos::log!(
            "New value for fail: {}",
            variables.fail_numerator.get()
        );
        if pass_updated.get_value() {
            pass_updated.set_value(false);
            return;
        }
        fail_updated.set_value(true);
        leptos::log!(
            "Setting pass: {}",
            variables.denominator.get_untracked()
                - variables.fail_numerator.get()
        );
        variables.pass_numerator.set(
            variables.denominator.get_untracked()
                - variables.fail_numerator.get(),
        );
    });

    create_effect(move |_| {
        leptos::log!(
            "New value for pass: {}",
            variables.pass_numerator.get()
        );
        if fail_updated.get_value() {
            fail_updated.set_value(false);
            return;
        }
        leptos::log!(
            "Setting fail: {}",
            variables.denominator.get_untracked()
                - variables.pass_numerator.get()
        );
        pass_updated.set_value(true);
        variables.fail_numerator.set(
            variables.denominator.get_untracked()
                - variables.pass_numerator.get(),
        );
    });

    view! {
        <div class="px-2 flex flex-col">
            <p class="text-center mb-2 text-3xl font-bold">
                <span class="text-red-500">
                    Small Disclaimer:
                </span>
                {"These algorithms are not optimized."}
            </p>
            <p class="text-center mb-2 text-xl">
                Link to source code(only for brave hearts):
                <a
                    class="text-blue-700"
                    href="https://github.com/PalaBeaveR/bernoulli_vs_moivre_laplace"
                >
                    {"https://github.com/PalaBeaveR/bernoulli_vs_moivre_laplace"}
                </a>
            </p>
            <Variables variables/>
            <DerivedVariables variables=derived_variables/>
            <div>
                <input
                    type="checkbox"
                    id="automaticiterations"
                    prop:checked=variables.automatic_iterations
                    on:input=move |ev| {
                        variables
                            .automatic_iterations
                            .set(event_target::<HtmlInputElement>(&ev).checked())
                    }
                />

                <label for="automaticiterations">
                    Automatic Iterations
                </label>
            </div>
            <p>
                {"Automatic iterations allow the algorithm to infer if it should continue iterating the exponent taylor series during calculation so that you don't get nonsensical results and then realize that you needed a bigger iteration number.(performance affected by precision variable)"}
            </p>
            <button
                on:click=move |_| {
                    let request = SolverRequest {
                        total: variables.total_experiments.get_untracked(),
                        required: variables.required_to_pass.get_untracked(),
                        odds: (
                            variables.pass_numerator.get_untracked(),
                            variables.denominator.get_untracked(),
                        ),
                        precision: variables.precision.get_untracked(),
                        iterations: variables.iterations.get_untracked(),
                        stable_amount: variables.stable_amount.get_untracked(),
                        sqrt_iterations: variables.sqrt_iterations.get_untracked(),

                    };
                    bernoulli_solver.send(request.clone());
                    set_bernoulli_running(true);
                    moivre_laplace_solver
                        .send((
                            if variables.automatic_iterations.get_untracked() {
                                MoivreLaplaceMode::AutomaticIterations
                            } else {
                                MoivreLaplaceMode::Normal
                            },
                            request,
                        ));
                    set_moivre_laplace_running(true);
                }

                class="bg-blue-500 rounded mt-2 h-10 text-2xl"
            >
                Calculate
            </button>
            <p>
                Iterations done(moivre laplace):
                {iterations_done}
            </p>
            <div class="grid grid-cols-2 child:border-2 child:border-black gap-2 p-2 child:rounded child:grow child:p-2">
                <ResultDisplay
                    precision=variables.precision
                    result=bernoulli_result
                    running=bernoulli_running
                    label="Bernoulli"
                />
                <ResultDisplay
                    precision=variables.precision
                    result=moivre_laplace_result
                    running=moivre_laplace_running
                    label="Moivre Laplace"
                />
            </div>
        </div>
    }
}

#[derive(Clone, Copy)]
pub struct Variables {
    pub total_experiments: RwSignal<u32>,
    pub required_to_pass: RwSignal<u32>,
    pub denominator: RwSignal<u32>,
    pub pass_numerator: RwSignal<u32>,
    pub fail_numerator: RwSignal<u32>,
    pub precision: RwSignal<usize>,
    pub iterations: RwSignal<u32>,
    pub stable_amount: RwSignal<usize>,
    pub automatic_iterations: RwSignal<bool>,
    pub sqrt_iterations: RwSignal<usize>
}

#[derive(Clone, Copy)]
pub struct DerivedVariables {
    pub npq: Signal<GenericFraction<BigUint>>,
    pub np: Signal<GenericFraction<BigUint>>,
}

#[component]
fn DerivedVariables(
    variables: DerivedVariables,
) -> impl IntoView {
    view! {
        <div class="flex flex-wrap justify-around child:px-2">
            <DerivedVariable
                value=move || format!("{:.50}", variables.np.get())
                id="np"
                label="np"
                block=true
            />
            <DerivedVariable
                value=move || format!("{:.50}", variables.npq.get())
                id="npq"
                label="npq"
                block=true
            />
        </div>
    }
}

#[component]
fn Variables(variables: Variables) -> impl IntoView {
    view! {
        <div class="flex flex-wrap justify-around child:px-2">
            <Variable
                value=variables.total_experiments
                id="total_experiments"
                label="Total Experiments(n)"
                block=true
            />
            <Variable
                value=variables.required_to_pass
                id="required_to_pass"
                label="Required To Pass(k)"
                block=true
            />
            <FractionVariable
                numerator=variables.pass_numerator
                denominator=variables.denominator
                block=true
                label="Pass Probability(p)"
            />
            <FractionVariable
                numerator=variables.fail_numerator
                denominator=variables.denominator
                block=true
                label="Fail Probability(q)"
            />

            <Variable
                value=variables.precision
                id="precision"
                label="Precision(Numbers after the dot in the result)"
                block=true
            />

            <Variable
                value=variables.sqrt_iterations
                id="sqrt"
                label="Sqrt Iterations(exponential time to compute)"
                block=true
            />

            {move || {
                if variables.automatic_iterations.get() {
                    view! {
                        <Variable
                            value=variables.stable_amount
                            id="stableamount"
                            label="Stable Number Amount(Only for moivre laplace. Affects how many first non-zero digits need to be the same from the previous iteration)"
                            block=true
                        />
                    }
                } else {
                    view! {
                        <Variable
                            value=variables.iterations
                            id="iterations"
                            label="Exponent Iterations(only affects moivre laplace. Bigger is slower but more accurate)"
                            block=true
                        />
                    }
                }
            }}

        </div>
    }
}

use std::str::FromStr;

#[component]
pub fn DerivedVariable<N>(
    #[prop(into)] value: Signal<N>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] block: bool,
) -> impl IntoView
where
    N: Clone + IntoView + 'static,
{
    let variable = view! {
        {label.map(|label| view! { <label for=id>{label}</label> }).collect_view()}
        <div id=id class="border-2 text-center rounded py-1">

            {move || value.get()}
        </div>
    };

    if block {
        view! { <div class="flex flex-col items-center">{variable}</div> }.into_view()
    } else {
        variable.into_view()
    }
}

#[component]
pub fn Variable<N>(
    value: RwSignal<N>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] block: bool,
) -> impl IntoView
where
    N: FromStr + Default + Clone + IntoView + 'static,
{
    use web_sys::HtmlDivElement;
    let variable = view! {
        {label.map(|label| view! { <label for=id>{label}</label> }).collect_view()}
        <div
            contenteditable
            id=id
            on:focusout=move |ev| {
                value
                    .set(
                        event_target::<HtmlDivElement>(&ev)
                            .inner_text()
                            .trim()
                            .parse()
                            .unwrap_or_default(),
                    )
            }

            on:keypress=move |ev| {
                let key = ev.code();
                if key == "Enter" {
                    ev.prevent_default();
                    event_target::<HtmlDivElement>(&ev).blur().unwrap();
                }
            }

            class="hover:border-red-500 border-2 text-center rounded py-1"
        >

            {move || value.get()}
        </div>
    };

    if block {
        view! { <div class="flex flex-col items-center">{variable}</div> }.into_view()
    } else {
        variable.into_view()
    }
}

#[component]
pub fn FractionVariable(
    numerator: RwSignal<u32>,
    denominator: RwSignal<u32>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] block: bool,
) -> impl IntoView {
    let variable = view! {
        {label.map(|label| view! { <p>{label}</p> })}
        <div class="flex flex-col items-center child:w-full w-min">
            <Variable value=numerator/>
            <hr class="my-1 h-[2px] bg-black"/>
            <Variable value=denominator/>
        </div>
    };

    if block {
        view! { <div class="flex flex-col items-center">{variable}</div> }.into_view()
    } else {
        variable.into_view()
    }
}

fn to_fraction(ratio: Ratio<BigUint>) -> BigFraction {
    let (numer, denum) = ratio.into();
    BigFraction::new(numer, denum)
}

#[component]
pub fn ResultDisplay(
    running: ReadSignal<bool>,
    result: ReadSignal<Option<SolverResult>>,
    precision: RwSignal<usize>,
    label: &'static str,
) -> impl IntoView {
    view! {
        <div>
            <div class="border-b-2 border-black flex justify-between items-center">
                <p>
                    Took:
                    {move || format!("{}", result.get().unwrap_or_default().took)}
                </p>
                <p>{label}</p>
                <p
                    class="bg-red-500 rounded px-[2px] py-px"
                    class=("!bg-green-500", move || running.get())
                >
                    {move || running.get().then_some("Running").unwrap_or("Idling")}
                </p>
            </div>
            <p class="break-words">
                {move || {
                    let precision = precision.get() as usize;
                    scientific_notation(result.get().unwrap_or_default().probability, precision)
                }}

            </p>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(|| {
        view! {
            <div class="w-screen h-screen flex flex-col overflow-x-hidden">
                <App/>
            </div>
        }
    })
}

pub fn scientific_notation(
    rat: GenericFraction<BigUint>,
    precision: usize,
) -> String {
    let st = format!("{:.precision$}", rat);

    let mut trimmed: String = st
        .chars()
        .skip_while(|ch| ch == &'0' || ch == &'.')
        .collect();
    let zeroes = st.len() - 1 - trimmed.len();

    match trimmed.len() {
        0 => {
            return "0".into();
        }
        2.. => trimmed.insert(1, '.'),
        _ => {}
    }

    format!("E-{} * {}", zeroes, trimmed)
}
