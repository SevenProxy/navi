mod components;
mod bootloader;
mod system;
mod binare;

use yew::prelude::*;

use bootloader::{
    StartRoot,
    PropsStart,
};
use components::{
    header::HeaderRoot,
    navbar::NavbarRoot,
};

use system::{
    Htop,
    PantheonDesktopRoot,
};

#[component]
fn App() -> Html {
    let bootloader_start = use_state(|| true);

    let props_start = yew::props! {
        PropsStart {
            state: bootloader_start.clone(),
        }
    };

    html!{
        <div>
            if *bootloader_start {
                <StartRoot ..props_start/>
            } else {
                <div class="bg-black h-screen overflow-hidden">
                    <div class="h-full">
                        <HeaderRoot />
                        <main class="h-full">
                            <PantheonDesktopRoot>
                                <Htop/>
                            </PantheonDesktopRoot>
                        </main>
                        <NavbarRoot />
                    </div>
                </div>
            }
        </div>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
