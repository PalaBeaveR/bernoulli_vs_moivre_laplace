use gloo_worker::Spawnable;
use leptos::svg::view;
use leptos::ReadSignal;
use leptos::{
    create_effect, event_target, spawn_local, store_value,
    watch, CollectView, RwSignal,
};

use leptos::{
    component, create_signal, event_target_value,
    leptos_dom::console_log, view, IntoView, SignalGet,
    SignalGetUntracked, SignalSet, SignalUpdate,
};
use serde::{Deserialize, Serialize};
use solver::{bernoulli, SolverResult};

use bernoulli_vs_moivre_laplace::{
    BernoulliSolver, MoivreLaplaceSolver, SolverRequest,
};

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

    let moivre_laplace_solver =
        MoivreLaplaceSolver::spawner()
            .callback(move |result| {
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
    };

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
        <p class="text-center mb-2 text-3xl font-bold">
            <span class="text-red-500">
                Small Disclaimer:
            </span>
            {"I don't even know if my implementation is correct. And it is definitelly not efficient."}
        </p>
        <p class="text-center mb-2 text-xl">
            Link to source code:
            <a
                class="text-blue-700"
                href="https://github.com/PalaBeaveR/bernoulli_vs_moivre_laplace"
            >
                {"https://github.com/PalaBeaveR/bernoulli_vs_moivre_laplace"}
            </a>
        </p>
        <Variables variables/>
        <button
            on:click=move |_| {
                let request = SolverRequest {
                    total: variables.total_experiments.get_untracked(),
                    required: variables.required_to_pass.get_untracked(),
                    odds: (
                        variables.pass_numerator.get_untracked(),
                        variables.denominator.get_untracked(),
                    ),
                };
                bernoulli_solver.send(request.clone());
                set_bernoulli_running(true);
                moivre_laplace_solver.send(request);
                set_moivre_laplace_running(true);
            }

            class="bg-blue-500 rounded mt-2 h-10 text-2xl"
        >
            Calculate
        </button>
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
    }
}

#[derive(Clone, Copy)]
pub struct Variables {
    pub total_experiments: RwSignal<u32>,
    pub required_to_pass: RwSignal<u32>,
    pub denominator: RwSignal<u32>,
    pub pass_numerator: RwSignal<u32>,
    pub fail_numerator: RwSignal<u32>,
    pub precision: RwSignal<u32>,
}

#[component]
fn Variables(variables: Variables) -> impl IntoView {
    view! {
        <div class="flex flex-wrap justify-around child:px-2">
            <Variable
                value=variables.total_experiments
                id="total_experiments"
                label="Total Experiments"
                block=true
            />
            <Variable
                value=variables.required_to_pass
                id="required_to_pass"
                label="Required To Pass"
                block=true
            />
            <FractionVariable
                numerator=variables.pass_numerator
                denominator=variables.denominator
                block=true
                label="Pass Probability"
            />
            <FractionVariable
                numerator=variables.fail_numerator
                denominator=variables.denominator
                block=true
                label="Fail Probability"
            />

            <Variable value=variables.precision id="precision" label="Precision" block=true/>
        </div>
    }
}

#[component]
pub fn Variable(
    value: RwSignal<u32>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] block: bool,
) -> impl IntoView {
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

            class="hover:border-red-500 border-2 text-center rounded px-2 py-1"
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

#[component]
pub fn ResultDisplay(
    running: ReadSignal<bool>,
    result: ReadSignal<Option<SolverResult>>,
    precision: RwSignal<u32>,
    label: &'static str,
) -> impl IntoView {
    view! {
        <div>
            <div class="border-b-2 border-black flex justify-between items-center p-1">
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
                    format!("{:.precision$}", result.get().unwrap_or_default().probability)
                }}

            </p>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(|| {
        view! {
            <div class="w-screen h-screen flex flex-col">
                <App/>
            </div>
        }
    })
}
