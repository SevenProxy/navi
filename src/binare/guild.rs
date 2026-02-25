use yew::prelude::*;
use crate::system::{
    WindowLucyRoot,
    PropsWindowLucy,
};
use super::PropsBinare;

#[component]
pub fn Guild(props: &PropsBinare) -> Html {
    let window_guild_props = yew::props! {
        PropsWindowLucy {
            name_window: "Discord".to_string(),
            pid: props.pid.clone(),
            style_custom: "z-0 max-w-[400px]".to_string(),
            sub_style: "w-full h-full text-white".to_string(),
        }
    };

    html! {
        <WindowLucyRoot ..window_guild_props>
            <div class="bg-black py-2 px-2">
                <div class="mb-2">
                    <img class="w-full max-2-[400px] h-[200px]" src="https://media2.giphy.com/media/v1.Y2lkPTc5MGI3NjExY3BkZnZmajFwNHhlYjIxenF0cTc4dGhmZXh0dWF6cWxoYWdkd3MwYiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/kqGp0mQGvdsg2MEiEr/giphy.gif" />
                </div>

                <div class="flex flex-col items-center gap-1">
                    <p class="text-white font-bold text-base">{"Entre no nosso servidor no discord, porra!"}</p>
                    <a class="text-blue-600 underline font-base" href="">{"tá..."}</a>
                </div>
            </div>
        </WindowLucyRoot>
    }
}
