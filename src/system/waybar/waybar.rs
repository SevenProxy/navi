use yew::prelude::*;
use web_sys::{
    wasm_bindgen::JsCast,
    HtmlElement,
};
use crate::{
    Management,
    software::{
        LocalSoftware,
        get_menu_select,
    },
};

#[component]
pub fn Waybar() -> Html {
    let window_start = use_state(|| false);
    let menu_software = use_state(|| String::new());
    let state = use_context::<UseStateHandle<Management>>().expect("No ctx found");
    let local = LocalSoftware::execute(state);

    let onclick_start = {
        let current = window_start.clone();

        Callback::from(move |_| {
            match (*current).clone() {
                true => current.set(false),
                false => current.set(true),
            }
        })
    };

    let onclick_selct = {
        let current = menu_software.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let element = e
                .target()
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap();

            if let Some(v) = element.get_attribute("value") {
                current.set(v);
            }
        })
    };

    html! {
        <>
        if *window_start {
            <button onclick={&onclick_start} class="absolute z-30 top-0 left-0 w-full h-full bg-transparent"></button>
        }
        <header class="w-full h-fit p-0 relative z-40">
            if *window_start {
                <div class="w-full h-full">
                    <div class="px-2 py-2 z-40 absolute top-8 left-o bg-pink-300">
                        <ul class="flex items-start flex-col gap-2">
                            { for get_menu_select().iter().map(|v| html! {
                                <li key={&*v.name} class="flex items-center justify-between max-w-[200px] min-w-[200px] relative">
                                    <div class="flex items-center gap-2">
                                        {v.icon.clone()}
                                        <p class="font-bold text-base">{&*v.name}</p>
                                    </div>
                                    <svg fill="currentColor" class="bi bi-chevron-right w-4 h-4 text-black" viewBox="0 0 16 16">
                                        <path fill-rule="evenodd" d="M4.646 1.646a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L10.293 8 4.646 2.354a.5.5 0 0 1 0-.708"/>
                                    </svg>
                                    <button value={v.name.clone()} onclick={onclick_selct.clone()} class="absolute top-0 left-0 w-full h-full"></button>
                                    if *menu_software == &*v.name {
                                        <div class="absolute -right-[95px] top-0 z-40 bg-pink-300 py-2 px-4">
                                            <ul class="flex flex-col items-start gap-2">
                                                { for v.select.iter().map(|v| html! {
                                                    <li class="flex items-center gap-2 relative" key={v.name.clone()}>
                                                        <img class="w-4 h-4" src={v.icon.clone()} />
                                                        <p class="text-bold text-base">{&*v.name}</p>
                                                        <button name={v.name.clone()} onclick={local.clone()} class="absolute top-0 left-0 w-full h-full"></button>
                                                    </li>
                                                })}
                                            </ul>
                                        </div>
                                    }
                                </li>
                            })}
                        </ul>
                    </div>
                </div>
            }
            <div class="bg-pink-300 text-black flex items-center relative justify-start gap-2 max-h-[30px]">
                <div class="py-0 flex items-center justify-center">
                    <button onclick={&onclick_start} class="border-0">
                        <img class="w-[70px] max-h-[30px]" src="https://fauux.neocities.org/FloatingScreen.gif" />
                    </button>
                </div>
                <nav class="py-2">
                    <ul class="text-base font-bold flex items-center gap-5">
                        <li>{"Status"}</li>
                        <li>{"Discord"}</li>
                        <li>{"About"}</li>
                    </ul>
                </nav>
            </div>
        </header>
        </>
    }
}
