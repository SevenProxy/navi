use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[component]
pub fn Desktop(props: &Props) -> Html {

    html! {
        <div class="w-full h-full relative">
            <div class="text-white w-full h-full">
                <div class="py-4 px-4 w-full max-h-full h-full">
                    <div class="flex flex-wrap flex-col items-start">
                    
                        <button class="border-0 max-w-[50px] max-h-[50px] flex flex-col items-center text-center">
                            <img class="w-full h-full" src="https://cdn.terminaltrove.com/m/8edf284d-5ce4-4d52-ab31-350866aaa79e.png" alt="foot"/>
                            <p class="text-zinc-200">{"foot"}</p>
                        </button>

                    </div>
                </div>
            </div>
            { props.children.clone() }
        </div>
    }
}
