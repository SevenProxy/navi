use yew::prelude::*;
use std::collections::HashMap;
use crate::binare::{
    foot::{
        Foot,
        PropsFoot,
    },
    lain::Lain,
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
                        "lain" => html!{
                            <Lain key={k.clone()}/>
                        },
                        _ => html! { <></> }
                    }
                })
            }
        </div>
    }
}
