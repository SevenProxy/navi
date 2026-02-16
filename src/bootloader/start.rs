use yew::prelude::*;
use std::rc::Rc;
use gloo::timers::callback::Timeout;

#[derive(Clone, PartialEq)]
enum TypeLog {
    Ok,
    Falied,
    Null,
}

enum BootAction {
    Push(BootLine),
    Clear,
}

enum StartedLog {
    Ok(String),
    Falied(String),
    Null(String),
}

#[derive(Properties, PartialEq, Clone)]
pub struct PropsStart {
    pub state: UseStateHandle<bool>,
}

#[derive(Clone, PartialEq)]
pub struct BootLine {
    text: String,
    ok: TypeLog,
}


#[derive(Clone, PartialEq)]
struct BootState {
    lines: Vec<BootLine>,
}


impl Reducible for BootState {
    type Action = BootAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            BootAction::Push(line) => {
                let mut lines = self.lines.clone();
                lines.push(line);

                Self { lines }.into()
            }

            BootAction::Clear => Self { lines: Vec::new() }.into(),
        }
    }
}

fn run_started_log() -> Vec<StartedLog> {
    let lines = vec![
        StartedLog::Null("starting version beta 0.1.0".into()),
        StartedLog::Null("/dev/system/*: starging".into()),
        StartedLog::Null("cargo check was32-unknow-unknown".into()),
        StartedLog::Null("trunk build".into()),
        StartedLog::Ok("Stoped Dispatch Password Requests to Console Directory Watch.".into()),
        StartedLog::Ok("Started /home".into()),
        StartedLog::Ok("Reached target Network".into()),
        StartedLog::Ok("Reached target Times".into()),
        StartedLog::Ok("Reached target Paths".into()),
        StartedLog::Ok("Reached Emergency Shell".into()),
        StartedLog::Ok("Reached target Sockets".into()),
        StartedLog::Ok("Mounted target Temporary Directory (/bin)".into()),
        StartedLog::Null("starting network time synchronization.".into()),
        StartedLog::Ok("Geting LocalStorage.".into()),
        StartedLog::Ok("Completed.".into()),
    ];

    lines
}

#[component]
pub fn StartRoot(props: &PropsStart) -> Html {
    let states = use_reducer(|| BootState {
        lines: Vec::new(),
    });
    let timeout_ref = use_mut_ref(|| Vec::<Timeout>::new());

    {
        let states = states.dispatcher();
        let timeout_ref = timeout_ref.clone();
        let props = props.clone();

        use_effect_with((), move |_|  {
            let delay_step = 600;
            let logs = run_started_log();
            let total_log = logs.len();

            for (i, v) in logs.into_iter().enumerate() {
                let dispatch = states.clone();
                let props = props.clone();

                let is_last = i == total_log - 1;

                let time_closure = Timeout::new(i as u32 * delay_step, move || {
                    match &v {
                        StartedLog::Ok(text) => {
                            dispatch.dispatch(BootAction::Push(BootLine {
                                text: text.to_string(),
                                ok: TypeLog::Ok,
                            }));
                        },

                        StartedLog::Falied(text) => {
                            dispatch.dispatch(BootAction::Push(BootLine {
                                text: text.to_string(),
                                ok: TypeLog::Falied,
                            }));
                        },

                        StartedLog::Null(text) => {
                            dispatch.dispatch(BootAction::Push(BootLine {
                                text: text.to_string(),
                                ok: TypeLog::Null,
                            }));
                        },
                    }

                    if is_last {
                        props.state.set(false);
                    }
                });

                timeout_ref.borrow_mut().push(time_closure);
            }

            move || {
                timeout_ref.borrow_mut().clear();
            }
        });
    }

    html! {
        <div class="z-50 h-screen w-full overflow-hidden bg-black text-white">
            <div class="w-full h-full">
                {
                    for states.lines.iter().map(|m| html! {
                        <div class="flex items-center gap-2">
                            { match &m.ok {
                                TypeLog::Ok => html! {
                                    <div class="text-bold flex items-center gap-2">
                                        <span>{"["}</span>
                                        <p class="text-green-600 text-bold">{"OK"}</p>
                                        <span>{"]"}</span>
                                    </div>
                                },

                                TypeLog::Falied => html! {
                                    <div class="text-bold flex items-center gap-2">
                                        <span>{"["}</span>
                                        <p class="text-red-600 text-bold">{"Falied"}</p>
                                        <span>{"]"}</span>
                                    </div>
                                },

                                TypeLog::Null => html! {
                                    <div class="text-bold flex items-center gap-2">
                                    </div>
                                },
                            }}
                            <p class="text-zinc-300">{&m.text}</p>
                        </div>
                    })
                }
                <span class="animate-ping mt-0 text-lg">{"▮"}</span>
            </div>
        </div>
    }
}
