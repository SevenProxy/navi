use yew::prelude::*;
use crate::{
    Management,
    software::LocalSoftware,
    system::{
        WindowLucyRoot,
        PropsWindowLucy,
    },
};
use super::PropsBinare;

#[component]
pub fn About(props: &PropsBinare) -> Html {
    let state = use_context::<UseStateHandle<Management>>().expect("No ctx foud");
    let window_props_about = yew::props! {
        PropsWindowLucy {
            name_window: "about".to_string(),
            pid: props.pid.clone(),
            style_custom: "z-0 max-w-[400px]".to_string(),
            sub_style: "w-full h-full text-white".to_string(),
        }
    };
    let local = LocalSoftware::close(state, props.pid.clone());

    html! {
        <WindowLucyRoot ..window_props_about>
            <div class="bg-black flex items-center justify-center text-center">
                <div class="py-2 px-2">
                    <div>
                        <h1 class="text-3xl underline font-about">{"ABOUT"}</h1>
                    </div>
                    <div class="my-2">
                        <p>{"Foda-se"}</p>
                        <button onclick={local} class="my-4 border-2 border-solid border-pink-500 text-base py-1 px-2">{"foda-se, proxy! cade as góticas"}</button>
                    </div>
                </div>
            </div>
        </WindowLucyRoot>
    }
}
