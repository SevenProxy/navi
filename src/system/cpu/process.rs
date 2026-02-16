use yew::prelude::*;
use std::collections::HashMap;
use crate::binare::{
    Foot,
    Lain,
};
use crate::system::local_storage::get_all_storage_values;

#[component]
pub fn Process() -> Html {
    let storage_data = use_state(HashMap::new);

    {
        let storage_data = storage_data.clone();
        use_effect_with((), move |_| {
            storage_data.set(get_all_storage_values());
            || ()
        });
    }

    html!{
        <div>
            {
                for storage_data.iter().map(|(k,v)| {
                    match v.as_str() {
                        "foot" => html!{
                            <Foot key={k.clone()} />
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
