use yew::prelude::*;
use chrono::{
    Utc,
    Local,
    DateTime,
};

#[component]
pub fn NavbarRoot() -> Html {
    let now = Local::now();
    let hour_formatted_time = now.format("%H:%M").to_string();
    let date_formatted_time = now.format("%Y/%m/%d").to_string();

    html!{
        <footer class="fixed z-10 bg-black bottom-1 left-0 w-full max-h-[40px]">
            <div class="w-full text-pink-300 h-full border-2 border-solid border-pink-300">
                <div class="flex items-center justify-between px-2 text-lg font-bold">
                    <div>
                        <p>{"Creator: 7proxy"}</p>
                    </div>
                    <span class="h-[40px] w-[2px] bg-pink-300" />
                    <div>
                        <p></p>
                    </div>
                    <span class="h-[40px] w-[2px] bg-pink-300"/>
                    <div>
                        <p>{"Languagem: BR"}</p>
                    </div>
                    <span class="h-[40px] w-[2px] bg-pink-300"/>
                    <div class="flex gap-2 items-center">
                        <p>{&hour_formatted_time}</p>
                        <p>{&date_formatted_time}</p>
                    </div>
                </div>
            </div>
        </footer>
    }
}
