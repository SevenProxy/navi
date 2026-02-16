use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[component]
pub fn Desktop(props: &Props) -> Html {

    html! {
        <div class="w-full h-full relative">
            { props.children.clone() }
        </div>
    }
}
