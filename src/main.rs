mod components;
mod bootloader;
mod system;
mod binare;
mod software;

use yew::prelude::*;
use std::collections::HashMap;
use bootloader::{
    StartRoot,
    PropsStart,
};
use components::navbar::NavbarRoot;
use system::{
    Waybar,
    Process,
    Desktop,
};

#[derive(Clone, PartialEq)]
struct Management {
    pub pid: HashMap<String, String>,
}

#[component]
fn App() -> Html {
    let bootloader_start = use_state(|| true);
    let storage_data = use_state(|| Management {
        pid: HashMap::new(),
    });

    let props_start = yew::props! {
        PropsStart {
            state: bootloader_start.clone(),
        }
    };

    html!{
        <ContextProvider<UseStateHandle<Management>> context={storage_data}>
            if false {
                <StartRoot ..props_start/>
            } else {
                <div class="bg-black h-screen overflow-hidden">
                    <div class="h-full">
                        <Waybar />
                        <main class="h-full">
                            <Desktop>
                                <Process/>
                            </Desktop>
                        </main>
                        <NavbarRoot />
                    </div>
                </div>
            }
        </ContextProvider<UseStateHandle<Management>>>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
