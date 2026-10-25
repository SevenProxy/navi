use yew::prelude::*;
use crate::system::{
    WindowLucyRoot,
    PropsWindowLucy,
};
use super::PropsBinare;

#[component]
pub fn Bin(props: &PropsBinare) -> Html {
    let window_props_bin = yew::props! {
        PropsWindowLucy {
            name_window: "bin".to_string(),
            pid: props.pid.clone(),
            style_custom: "z-0 max-w-[350px]".to_string(),
            sub_style: "w-full h-full text-white".to_string(),
        }
    };

    html! {
        <WindowLucyRoot ..window_props_bin>
            <div class="bg-black py-2 px-2">
                <div class="mb-2 text-center">
                    <p class="text-pink-500 font-bold">{"[ bin ]"}</p>
                </div>
                <div class="flex flex-col items-center gap-2 text-sm">
                    <p>{"Executáveis disponíveis:"}</p>
                    <ul class="text-left">
                        <li>{"foot"}</li>
                        <li>{"lain"}</li>
                        <li>{"lain_music"}</li>
                        <li>{"about"}</li>
                        <li>{"guild"}</li>
                    </ul>
                    <p class="text-xs text-gray-400 mt-2">{"Use o foot para executar"}</p>
                </div>
            </div>
        </WindowLucyRoot>
    }
}
