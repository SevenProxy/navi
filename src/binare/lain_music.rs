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
            name_window: "lain_music".to_string(),
            pid: props.pid.clone(),
            style_custom: "max-w-[300px] max-h-[200px] z-0".to_string(),
            sub_style: "w-full h-full text-white".to_string(),
        }
    };


    html!{
        <WindowLucyRoot ..window_props_lain>
            <div class="bg-black w-full flex flex-col items-center justify-center">
                <div class="py-2 px-2 text-sm">
                    <p>{"Ainda vou por mais algumas músicas minhas, mas por agora só vai ter essa :D"}</p>
                    <p>{"Se tu nao gosta de phonk problema é seu taligado"}</p>
                    <br/>
                    <p>{"Falando nisso, esse programa aqui vai ser tipo um \"Spotify\", se você tiver sugestões de músicas manda lá no servidor do discord. Onde eu acho? só clica no icon na barra superior ali em cima"}</p>
                </div>
                <audio controls={true}>
                    <source src="public/starly&jessie.mp3" type="audio/ogg"/>
                </audio>
            </div>
        </WindowLucyRoot>
    }
}
