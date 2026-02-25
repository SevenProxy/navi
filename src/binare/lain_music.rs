use yew::prelude::*;
use crate::system::{
    WindowLucyRoot,
    PropsWindowLucy,
};
use super::PropsBinare;

#[component]
pub fn LainMusic(props: &PropsBinare) -> Html {
    let window_props_lain = yew::props! {
        PropsWindowLucy {
            name_window: "LainMusic".to_string(),
            pid: props.pid.clone(),
            style_custom: "max-w-[300px] max-h-[200px] z-0".to_string(),
            sub_style: "w-full h-full text-white".to_string(),
        }
    };


    html!{
        <WindowLucyRoot ..window_props_lain>
            <div class="w-full flex items-center justify-center">
                <audio controls={true}>
                    <source src="public/starly&jessie.mp3" type="audio/ogg"/>
                </audio>
            </div>
        </WindowLucyRoot>
    }
}
