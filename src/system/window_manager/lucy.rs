use yew::prelude::*;
use crate::{
    Management,
    software::LocalSoftware,
    system::local_storage::{
        get_all_storage_values,
    },
};

#[derive(Properties, PartialEq, Clone)]
pub struct PropsWindowLucy {
    #[prop_or_default]
    pub children: Children,

    pub pid: String,

    pub name_window: String,

    pub style_custom: String,

    pub sub_style: String,
}

#[component]
pub fn WindowLucyRoot(props: &PropsWindowLucy) -> Html {
    let position = use_state(|| (100, 100));
    let dragging = use_state(|| false);
    let last_mouse = use_state(|| (0, 0));
    let state = use_context::<UseStateHandle<Management>>().expect("No ctx found.");

    let on_close = LocalSoftware::close(state.clone(), props.pid.clone());

    let on_mousedown = {
        let dragging = dragging.clone();
        let last_mouse = last_mouse.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            dragging.set(true);
            last_mouse.set((e.client_x(), e.client_y()))
        })
    };

    let on_mouseup = {
        let dragging = dragging.clone();
        Callback::from(move |_| {
            dragging.set(false);
        })
    };

    let on_mousemove = {
        let dragging = dragging.clone();
        let position = position.clone();
        let last_mouse = last_mouse.clone();

        Callback::from(move |e: MouseEvent| {
            if *dragging {
                let (lx, ly) = *last_mouse;
                let dx = e.client_x() - lx;
                let dy = e.client_y() - ly;

                let (x, y) = *position;
                position.set((x + dx, y + dy));
                last_mouse.set((e.client_x(), e.client_y()));
            }
        })
    };

    let on_click = {
        let state = state.clone();
        let props = props.clone();
        Callback::from(move |_| {
            let current = Management {
                pid: get_all_storage_values(),
                soft_current: Some(props.name_window.clone()),
            };
            state.set(current)
        })
    };

    let style = format!(
        "left: {}px; top: {}px;",
        position.0, position.1
    );

    let class_window = format!("font-bold text-base absolute z-10 bg-back sway-color-border {}", &props.style_custom);

    html!{
        <section
            class={&class_window}
            onmousemove={on_mousemove}
            onmouseup={on_mouseup}
            onclick={on_click}
            style={style}
        >
            <div class="h-[30px] bg-black cursor-move flex items-center justify-between gap-2 py-2 px-2" onmousedown={on_mousedown}>
                /*<div class="flex items-center gap-2">
                    <span class="h-[25px] w-[25px] border-[7px] border-solid border-pink-600 bg-pink-300"/>
                    <span class="h-[25px] w-[25px] border-[7px] border-solid border-pink-600 bg-pink-300"/>
                </div>*/
                <div class="flex items-center justify-between gap-2 w-full">
                    <span class="h-2 w-full sway-color-bg"/>
                    <p class="whitespace-nowrap text-white">{"[ "} {&props.name_window} {format!("@{}", &props.pid)} {" ]"}</p>
                    <span class="h-2 w-full sway-color-bg"/>
                </div>
                <button onclick={on_close} class="text-3xl font-bold flex items-center justify-center text-center h-[25px] sway-color">{"/"}</button>
            </div>
            <div class={&props.sub_style}>
                { for props.children.iter() }
            </div>
        </section>
    }
}
