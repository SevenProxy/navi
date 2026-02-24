use yew::prelude::*;
use std::collections::HashMap;
use crate::binare::{
    foot::{
        Foot,
        PropsFoot,
    },
    lain::{
        Lain,
        PropsLain,
    },
    about::{
        About,
        PropsAbout,
    },
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
                                PropsFoot {
                                    pid: k.clone(),
                                }
                            };

                            html! {
                                <Foot key={k.clone()} ..foot_pid/>
                            }
                        },
                        "lain" => {
                            let lain_pid = yew::props! {
                                PropsLain {
                                    pid: k.clone(),
                                }
                            };

                            html! {
                                <Lain key={k.clone()} ..lain_pid/>
                            }
                        },
                        "about" => {
                            let about_pid = yew::props! {
                                PropsAbout {
                                    pid: k.clone(),
                                }
                            };

                            html! {
                                <About key={k.clone()} ..about_pid/>
                            }
                        },
                        _ => html! { <></> }
                    }
                })
            }
        </div>
    }
}
