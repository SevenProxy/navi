use yew::prelude::*;
use crate::system::WindowLucyRoot;
use crate::system::PropsWindowLucy;

#[derive(Properties, PartialEq, Clone)]
pub struct PropsLain {
    pub pid: String,
}

#[component]
pub fn Lain(props: &PropsLain) -> Html {
    let window_props_lain = yew::props! {
        PropsWindowLucy {
            name_window: "Lain".to_string(),
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
