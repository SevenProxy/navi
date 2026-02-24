use yew::prelude::*;
use crate::{
    Management,
    software::LocalSoftware,
    system::{
        WindowLucyRoot,
        PropsWindowLucy,
    },
};

#[derive(Properties, PartialEq, Clone)]
pub struct PropsAbout {
    pub pid: String,
}

#[component]
pub fn About(props: &PropsAbout) -> Html {
    let state = use_context::<UseStateHandle<Management>>().expect("No ctx foud");
    let window_props_about = yew::props! {
        PropsWindowLucy {
            name_window: "About".to_string(),
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
                        <p>{"O projeto tem licença MIT e GNU. O projeto se encontra no github, não vou por o link aqui, cabe a você procurar ele, não sou seu seus país para dar tudo na sua mão."}</p>
                        <p>{"Esse projeto é algo talvez para estudo, mas também para testar tecnologias poucos usadas como webassembly. Quero ver ate onde vou com isso."}</p>
                        <p>
                            {"A ideia inicial era fazer um gerenciador de servidores com a interface de um sistema operacional, algo minimamente criativo kkk, mas acabou sendo isso... talvez futuramente eu faça o que quero, "}
                            <span class="bg-pink-500">{"mas quero deixar isso ativo pra ver qual vai ser."}</span>
                        </p>
                        <button onclick={local} class="my-4 border-2 border-solid border-pink-500 text-base py-1 px-2">{"foda-se, proxy! cade as góticas"}</button>
                    </div>
                </div>
            </div>
        </WindowLucyRoot>
    }
}
