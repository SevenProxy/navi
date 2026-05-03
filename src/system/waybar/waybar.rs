use gloo::timers::callback::Interval;
use yew::prelude::*;
use web_sys::{
    HtmlElement,
    window,
};
use chrono::Local;
use wasm_bindgen::JsCast;

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
    let memory_usage = use_state(|| 0);
    let state = use_context::<UseStateHandle<Management>>().expect("No ctx found");
    let local = LocalSoftware::execute(state.clone());
    let now = Local::now();
    let hour_formatted_time = now.format("%H:%M").to_string();
    let date_formatted_time = now.format("%Y/%m/%d").to_string();


    let onclick_start = {
        let current = window_start.clone();

        Callback::from(move |_| {
            current.set(!*current);
        })
    };

    let onclick_selct = {
        let current = menu_software.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if let Some(element) = e.target() {
                let element_current = element.dyn_into::<HtmlElement>();

                match element_current {
                    Ok(t) => if let Some(v) = t.get_attribute("value") {
                        if (*current) == v {
                            current.set("".to_string());
                        } else {
                            current.set(v);
                        }
                    },
                    Err(_) => {}
                }
            }
        })
    };

    {
        let memory_usage = memory_usage.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(1000, move || {
                if let Some(window) = window () {
                    if let Ok(perf) = window.performance().ok_or("no perf").map(|p| p) {
                        let memory = js_sys::Reflect::get(&perf, &"memory".into()).unwrap();
                        let used_heap = js_sys::Reflect::get(&memory, &"usedJSHeapSize".into()).unwrap();
                        let mb = (used_heap.as_f64().unwrap() / 1024.0 / 1024.0) as u32;

                        memory_usage.set(mb);
                    }
                }
            });

            Box::new(move || drop(interval))
        })
    }

    html! {
        <>
        if *window_start {
            <button onclick={&onclick_start} class="absolute z-30 top-0 left-0 w-full h-full bg-transparent"></button>
        }
        <header class="w-full h-fit p-0 relative z-40">
            if *window_start {
                <div class="w-full h-full">
                    <div class="px-2 py-2 z-40 absolute top-8 left-o bg-black sway-color-border text-white">
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
                                        <div class="max-w-[200px] min-w-[200px] overflow-hidden absolute -right-[210px] top-0 z-40 bg-black border-2 border-solid border-[#707880] text-white py-2 px-4">
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
            <div class="bg-black text-white flex items-center relative justify-between min-h-[40px] max-h-[40px]">
                <div class="py-0 flex items-center justify-center">
                    <button onclick={&onclick_start} class="border-0">
                        <img class="w-[70px] max-h-[30px]" src="public/img/start_menu.gif" />
                    </button>
                    <div class="flex items-center gap-2 sway-color font-sm font-bold">
                        <svg fill="currentColor" class="bi bi-stopwatch w-4 h-4" viewBox="0 0 16 16">
                            <path d="M8.5 5.6a.5.5 0 1 0-1 0v2.9h-3a.5.5 0 0 0 0 1H8a.5.5 0 0 0 .5-.5z"/>
                            <path d="M6.5 1A.5.5 0 0 1 7 .5h2a.5.5 0 0 1 0 1v.57c1.36.196 2.594.78 3.584 1.64l.012-.013.354-.354-.354-.353a.5.5 0 0 1 .707-.708l1.414 1.415a.5.5 0 1 1-.707.707l-.353-.354-.354.354-.013.012A7 7 0 1 1 7 2.071V1.5a.5.5 0 0 1-.5-.5M8 3a6 6 0 1 0 .001 12A6 6 0 0 0 8 3"/>
                        </svg>
                        <div class="flex gap-2 items-center">
                            <p>{&hour_formatted_time}</p>
                            <p>{&date_formatted_time}</p>
                        </div>
                    </div>
                </div>
                <div class="text-sm sway-color-text">
                    <p class="font-bold">{get_soft_current(state.clone())}</p>
                </div>
                <nav class="py-2 px-2">
                    <ul class="text-sm font-bold flex items-center gap-5">
                        <button name="guild" onclick={&local} class="bg-none border-0 sway-color flex items-center gap-2">
                            <svg fill="currentColor" class="bi bi-discord w-4 h-4" viewBox="0 0 16 16">
                                <path d="M13.545 2.907a13.2 13.2 0 0 0-3.257-1.011.05.05 0 0 0-.052.025c-.141.25-.297.577-.406.833a12.2 12.2 0 0 0-3.658 0 8 8 0 0 0-.412-.833.05.05 0 0 0-.052-.025c-1.125.194-2.22.534-3.257 1.011a.04.04 0 0 0-.021.018C.356 6.024-.213 9.047.066 12.032q.003.022.021.037a13.3 13.3 0 0 0 3.995 2.02.05.05 0 0 0 .056-.019q.463-.63.818-1.329a.05.05 0 0 0-.01-.059l-.018-.011a9 9 0 0 1-1.248-.595.05.05 0 0 1-.02-.066l.015-.019q.127-.095.248-.195a.05.05 0 0 1 .051-.007c2.619 1.196 5.454 1.196 8.041 0a.05.05 0 0 1 .053.007q.121.1.248.195a.05.05 0 0 1-.004.085 8 8 0 0 1-1.249.594.05.05 0 0 0-.03.03.05.05 0 0 0 .003.041c.24.465.515.909.817 1.329a.05.05 0 0 0 .056.019 13.2 13.2 0 0 0 4.001-2.02.05.05 0 0 0 .021-.037c.334-3.451-.559-6.449-2.366-9.106a.03.03 0 0 0-.02-.019m-8.198 7.307c-.789 0-1.438-.724-1.438-1.612s.637-1.613 1.438-1.613c.807 0 1.45.73 1.438 1.613 0 .888-.637 1.612-1.438 1.612m5.316 0c-.788 0-1.438-.724-1.438-1.612s.637-1.613 1.438-1.613c.807 0 1.451.73 1.438 1.613 0 .888-.631 1.612-1.438 1.612"/>
                            </svg>
                        </button>
                        <button name="about" onclick={&local} class="bg-none border-0 sway-color flex items-center gap-2">
                            <svg fill="currentColor" class="bi bi-file-person-fill w-4 h-4" viewBox="0 0 16 16">
                                <path d="M12 0H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2m-1 7a3 3 0 1 1-6 0 3 3 0 0 1 6 0m-3 4c2.623 0 4.146.826 5 1.755V14a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1v-1.245C3.854 11.825 5.377 11 8 11"/>
                            </svg>
                        </button>
                        <div class="flex sway-color items-center gap-2">
                            <svg fill="currentColor" class="bi bi-memory w-4 h-4" viewBox="0 0 16 16">
                                <path d="M1 3a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h4.586a1 1 0 0 0 .707-.293l.353-.353a.5.5 0 0 1 .708 0l.353.353a1 1 0 0 0 .707.293H15a1 1 0 0 0 1-1V4a1 1 0 0 0-1-1zm.5 1h3a.5.5 0 0 1 .5.5v4a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5v-4a.5.5 0 0 1 .5-.5m5 0h3a.5.5 0 0 1 .5.5v4a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5v-4a.5.5 0 0 1 .5-.5m4.5.5a.5.5 0 0 1 .5-.5h3a.5.5 0 0 1 .5.5v4a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5zM2 10v2H1v-2zm2 0v2H3v-2zm2 0v2H5v-2zm3 0v2H8v-2zm2 0v2h-1v-2zm2 0v2h-1v-2zm2 0v2h-1v-2z"/>
                            </svg>
                            {*memory_usage}
                        </div>
                    </ul>
                </nav>
            </div>
        </header>
        </>
    }
}

fn get_soft_current(state: UseStateHandle<Management>) -> Html {
    let state = (*state).clone();

    match state.soft_current {
        Some(name) => html!{<p>{name.clone()}</p>},
        None => html!{<p>{"No aba"}</p>}
    }
}
