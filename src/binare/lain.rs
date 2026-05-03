use yew::prelude::*;
use crate::system::{
    PropsWindowLucy,
    WindowLucyRoot,
};
use super::PropsBinare;

#[component]
pub fn Lain(props: &PropsBinare) -> Html {
    let window_props_lain = yew::props! {
        PropsWindowLucy {
            name_window: "lain".to_string(),
            pid: props.pid.clone(),
            style_custom: "max-w-[300px] max-h-[300px] z-0".to_string(),
            sub_style: "w-full h-full text-white".to_string(),
        }
    };


    html!{
        <WindowLucyRoot ..window_props_lain>
            <div class="bg-black">
                <img class="w-full h-full bg-black" src="https://fauux.neocities.org/16c.gif" />
            </div>
        </WindowLucyRoot>
    }
}
