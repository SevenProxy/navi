use yew::prelude::*;
use web_sys::{
    wasm_bindgen::JsCast,
    HtmlElement,
};
use js_sys::Math;
use crate::{
    Management,
    system::local_storage::{
        remove_item,
        set_item,
        get_all_storage_values,
    },
};

#[derive(Clone)]
pub struct LocalSoftware;

impl LocalSoftware {
    pub fn execute(state: UseStateHandle<Management>) -> yew::Callback<yew::MouseEvent> {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let element = e
                .target()
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap();

            if let Some(v) = element.get_attribute("name") {
                let random_pid = (Math::random() * 99999 as f64) as usize;
                let current = random_pid.to_string();

                set_item(current.as_str(), v.as_str());
                state.set(Management {
                    pid: get_all_storage_values(),
                });
            }
        })
    }

    pub fn close(state: UseStateHandle<Management>, pid: String) -> yew::Callback<yew::MouseEvent> {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            remove_item(pid.as_str());

            state.set(Management {
                pid: get_all_storage_values(),
            });
        })
    }
}
