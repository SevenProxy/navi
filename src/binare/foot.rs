use yew::prelude::*;
use web_sys::{
    HtmlInputElement,
    KeyboardEvent,
    console,
    wasm_bindgen::JsValue,
};
use std::collections::HashMap;
use crate::system::{
    WindowLucyRoot,
    PropsWindowLucy,
};
use super::PropsBinare;

#[derive(Debug)]
enum Command {
    Clear,
    Whoami,
    Help,
    Neofetch,
    Echo(String),
    Unknown(String),
}

fn neofetch_block() -> HashMap<String, Vec<String>> {
    let mut block = HashMap::new();
    let banner = vec![
        "         Lain@2026".into(),
        "⠀⠀⠀⠀⣀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡢⡀⠀⠀⠀⠀".into(),
        "⠀⠀⠀⢄⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦".into(),
        "⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧".into(),
        " ⢨⣿⡿⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣇".into(),
        " ⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢀".into(),
        "⢈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⢹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣾⠀".into(),
        "⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⡇⠀⢸⢹⣿⣿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡧".into(),
        "⠈⣿⣿⣿⣿⣿⠧⠯⠟⠿⠧⠀⠀⠀⠸⠿⠿⢼⣿⠿⢿⣟⣿⣿⣿⣿⣿⣇".into(),
        "⠀⣿⣿⣿⣿⡿⢰⠺⣿⠉⠂⠀⠀⠀⠀⠀⠀⠚⣷⣶⠢⡀⢿⣿⣿⣿⡿⠉".into(),
        "⢐⢻⣿⣏⠙⠇⠈⠒⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠝⠻⠥⠁⢰⡌⠹⠋⡀⡀".into(),
        "⠀⠉⢻⣿⣦⡀⠐⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠆⠄⠸⢃⣰⡀⠱".into(),
        "⠀⠀⠀⢹⣿⣿⡄⠀⠀⠀⠀⠀⠀⡀⡀⠀⠀⠀⠀⠀⠀⢀⣶⣿⣿⡟⠁".into(),
        "⠀⠀⠘⣸⢿⣿⣿⣦⡀⠀⠀⠀⠀⠠⠄⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣇".into(),
        "⠀⠀⠀⠉⠞⠿⠛⠿⠿⢶⣄⠀⠀⠀⠀⠀⠀⠀⣠⡾⠿⠿⣿⣿⡿⠅⠀⠀⠀".into(),
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣶⣤⣤⣤⡴⠊⠀⡧⠀⠀⣿⣿⣇".into(),
        "⠀⠀⠀⠀⠀⠀⠀⠀⣀⡞⠛⠿⣿⣿⠟⠋⠀⠀⠀⠱⣀⠈⣿⣿⡁".into(),
        "⠀⠀⠀⠀⠀⢠⡠⠔⠋⠀⠀⠀⠈⠁⠀⠀⠀⠀⠀⠀⠙⠲⣿⣧".into(),
        "⢀⠔⠒⠀⠉⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡿⠀⠉⠚⠤⢔".into(),
    ];

    block.insert(
        "neofetch".into(),
        banner,
    );

    block
}

fn parse_command(input: &str) -> Command {
    let input = input.trim();

    let mut parts = input.split_whitespace();
    let cmd = parts.next();

    match cmd {
        Some("clear") => Command::Clear,
        Some("whoami") => Command::Whoami,
        Some("help") => Command::Help,
        Some("neofetch") => Command::Neofetch,
        Some("echo") => {
            let reset = parts.collect::<Vec<_>>().join(" ");
            Command::Echo(reset)
        },
        Some(other) => Command::Unknown(other.to_string()),
        None => Command::Unknown(String::new()),
    }
}

fn run_command(
    cmd: Command,
    lines: UseStateHandle<Vec<HashMap<String, Vec<String>>>>
) {

    match cmd {
        Command::Clear => {
            lines.set(Vec::new());
        }

        Command::Whoami => {
            let mut new_blocks = (*lines).clone();

            let mut block = HashMap::new();
            block.insert("whoami".into(), vec!["proxy".into()]);

            new_blocks.push(block);
            lines.set(new_blocks);
        }

        Command::Neofetch => {
            let mut new_blocks = (*lines).clone();
            new_blocks.push(neofetch_block());
            lines.set(new_blocks);
        }

        Command::Unknown(cmd) => {
            let mut new_blocks = (*lines).clone();

            let mut block = HashMap::new();
            block.insert(cmd, vec!["Comando foi encontrado.".into()]);

            new_blocks.push(block);
            lines.set(new_blocks);
        }

        Command::Echo(txt) => {
            let mut new_blocks = (*lines).clone();

            let mut block = HashMap::new();
            block.insert("echo".into(), vec![format!("{}", txt)]);

            new_blocks.push(block);
            lines.set(new_blocks);
        }

        Command::Help => todo!(),

    }
}

#[component]
pub fn Foot(props: &PropsBinare) -> Html {
    let output_history = use_state(|| Vec::<HashMap<String, Vec<String>>>::new());
    let input_value = use_state(|| String::new());

    {
        let blocks = output_history.clone();
        use_effect_with((), move |_| {
            run_command(Command::Neofetch, blocks);
            || ()
        });
    }

    let window_props_terminal = yew::props! {
        PropsWindowLucy {
            name_window: "Terminal".to_string(),
            pid: props.pid.clone(),
            style_custom: "w-[600px] z-0".to_string(),
            sub_style: "bg-black overflow-y-auto overflow-x-hidden max-h-[500px] px-2 h-full text-white",
        }
    };

    let oninput = {
        let input_value = input_value.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };

    let onsubmit = {
        let output_history = output_history.clone();
        let input_value = input_value.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if input_value.is_empty() {
                return;
            }

            let command_cli = parse_command(&input_value.as_str());
            run_command(command_cli, output_history.clone());

            input_value.set(String::new());
        })
    };

    html!{
        <WindowLucyRoot ..window_props_terminal>
            <div>
                <div>
                    { for output_history.iter().flat_map(|m| {
                        m.iter().map(|(cmd, r)| html! {
                            <div class="w-full my-2">
                                <div class="my-2">
                                    <div class="flex items-center">
                                        <span class="text-pink-600">{"╭─"}</span>
                                        <span class="text-pink-600">{"proxy"}</span>
                                        <span>{"@"}</span>
                                        <span class="text-pink-800">{"Skynet"}</span>
                                        <span class="mx-1">{"in"}</span>
                                        <span class="text-pink-600">{"/home"}</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <span class="text-pink-600">{"╰─λ"}</span>
                                        <p class="w-full text-white">{ cmd.clone() }</p>
                                    </div>
                                    <div>
                                        { for r.iter().map(|line| html! {
                                            <p>{ line.clone() }</p>
                                        })}
                                    </div>
                                </div>
                            </div>
                        })
                    }) }
                </div>

                <div class="w-full my-2">
                    <div class="my-2">
                        <div class="flex items-center">
                            <span class="text-pink-600">{"╭─"}</span>
                            <span class="text-pink-600">{"proxy"}</span>
                            <span>{"@"}</span>
                            <span class="text-pink-800">{"Skynet"}</span>
                            <span class="mx-1">{"in"}</span>
                            <span class="text-pink-600">{"/home"}</span>
                        </div>
                        <form {onsubmit} class="flex items-center gap-1">
                            <span class="text-pink-600">{"╰─λ"}</span>
                            <input value={(*input_value).clone()} {oninput} autofocus=true class="bg-transparent border-0 w-full outline-none"/>
                        </form>
                    </div>
                </div>
            </div>
        </WindowLucyRoot>
    }
}
