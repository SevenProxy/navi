use yew::prelude::*;
use crate::system::{
    WindowLucyRoot,
    PropsWindowLucy,
};
use super::PropsBinare;

#[component]
pub fn Terminal(props: &PropsBinare) -> Html {
    let window_props_terminal = yew::props! {
        PropsWindowLucy {
            name_window: "terminal".to_string(),
            pid: props.pid.clone(),
            style_custom: "z-0 max-w-[400px]".to_string(),
            sub_style: "w-full h-full text-white".to_string(),
        }
    };

    html! {
        <WindowLucyRoot ..window_props_terminal>
            <div class="bg-black py-2 px-2 h-[200px] overflow-auto">
                <p class="text-green-400">{"[terminal@skynet ~]$ echo 'hello'"}</p>
                <p>{"hello"}</p>
                <p class="text-green-400">{"[terminal@skynet ~]$ _"}</p>
            </div>
        </WindowLucyRoot>
    }
}
