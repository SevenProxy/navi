use yew::prelude::*;
use crate::{
    Management,
    software::{
        LocalSoftware,
        get_software,
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[component]
pub fn Desktop(props: &Props) -> Html {
    let state = use_context::<UseStateHandle<Management>>().expect("No ctx foud");
    let onclick = LocalSoftware::execute(state.clone());

    html! {
        <div class="w-full h-full relative">
            <div class="text-white w-full h-full">
                <div class="py-4 px-4 w-full max-h-full h-full">
                    <div class="flex flex-wrap flex-col items-start">
                        { for get_software().iter().map(|v| html! {
                            <button class="mb-2 border-0 max-w-[50px] max-h-[50px] flex flex-col items-center text-center relative">
                                <button name={v.name.clone()} onclick={onclick.clone()} class="z-10 absolute top-0 left-0 w-full h-full"></button>
                                <img class="w-full h-full" src={v.icon.clone()} alt={v.name.clone()}/>
                                <p class="text-zinc-200">{v.name.clone()}</p>
                            </button>
                        })}
                    </div>
                </div>
            </div>
            { props.children.clone() }
        </div>
    }
}
