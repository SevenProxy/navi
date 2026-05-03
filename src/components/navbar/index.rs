use yew::prelude::*;

#[component]
pub fn NavbarRoot() -> Html {
    html!{
        <footer class="fixed z-10 bg-black bottom-1 left-0 w-full max-h-[35px]">
            <div class="w-full sway-color h-full border-2 sway-color-border">
                <div class="flex items-center justify-between px-2 text-sm font-bold">
                    <div>
                        <p>{"Discord: 7proxy"}</p>
                    </div>
                    <span class="h-[40px] w-[2px] sway-color-bg" />
                    <div>
                        <p>{"Version: 0.2.0"}</p>
                    </div>
                    <span class="h-[40px] w-[2px] sway-color-bg" />
                    <div>
                        <p>{"Tech: Rust + Webassembly"}</p>
                    </div>
                </div>
            </div>
        </footer>
    }
}
