use yew::prelude::*;
use crate::binare::{
    PropsBinare,
    foot::Foot,
    lain::Lain,
    about::About,
    guild::Guild,
    lain_music::LainMusic,
    bin::Bin,
    terminal::Terminal,
};
use crate::{
    Management,
    system::local_storage::get_all_storage_values,
};

#[component]
pub fn Process() -> Html {
    let state = use_context::<UseStateHandle<Management>>().expect("No ctx found");

    {
        let storage_data = state.clone();
        use_effect_with((), move |_| {
            storage_data.set( Management {
                pid: get_all_storage_values(),
                soft_current: None,
            });
            || ()
        });
    }

    html!{
        <div>
            {
                for state.pid.iter().map(|(k,v)| {
                    match v.as_str() {
                        "foot" => {
                            let foot_pid = yew::props! {
                                PropsBinare {
                                    pid: k.clone(),
                                }
                            };

                            html! {
                                <Foot key={k.clone()} ..foot_pid/>
                            }
                        },
                        "lain" => {
                            let lain_pid = yew::props! {
                                PropsBinare {
                                    pid: k.clone(),
                                }
                            };

                            html! {
                                <Lain key={k.clone()} ..lain_pid/>
                            }
                        },
                        "about" => {
                            let about_pid = yew::props! {
                                PropsBinare {
                                    pid: k.clone(),
                                }
                            };

                            html! {
                                <About key={k.clone()} ..about_pid/>
                            }
                        },
                        "guild" => {
                            let guild_pid = yew::props! {
                                PropsBinare {
                                    pid: k.clone(),
                                }
                            };

                            html! {
                                <Guild key={k.clone()} ..guild_pid/>
                            }
                        },
                        "lain_music" => {
                            let lain_music = yew::props! {
                                PropsBinare {
                                    pid: k.clone(),
                                }
                            };

                            html! {
                                <LainMusic key={k.clone()} ..lain_music/>
                            }
                        },
                        "bin" => {
                            let bin_props = yew::props! {
                                PropsBinare {
                                    pid: k.clone(),
                                }
                            };

                            html! {
                                <Bin key={k.clone()} ..bin_props/>
                            }
                        },
                        "terminal" => {
                            let terminal_props = yew::props! {
                                PropsBinare {
                                    pid: k.clone(),
                                }
                            };

                            html! {
                                <Terminal key={k.clone()} ..terminal_props/>
                            }
                        }
                        _ => html! { <></> }
                    }
                })
            }
        </div>
    }
}
