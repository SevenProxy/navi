use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::collections::HashMap;
use crate::{
    Management,
    system::{
        WindowLucyRoot,
        PropsWindowLucy,
        local_storage::{
            set_item,
            get_all_storage_values,
        },
    },
};
use super::PropsBinare;
use js_sys::Math;

#[derive(Debug)]
enum Command {
    Ls,
    Cat(String),
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
        Some("ls") => Command::Ls,
        Some("cat") => {
            let reset = parts.collect::<Vec<_>>().join(" ");
            Command::Cat(reset)
        },
        Some("clear") | Some("cls") => Command::Clear,
        Some("whoami") | Some("id") => Command::Whoami,
        Some("help") | Some("?") => Command::Help,
        Some("neofetch") | Some("fetch") | Some("info") => Command::Neofetch,
        Some("echo") => {
            let reset = parts.collect::<Vec<_>>().join(" ");
            Command::Echo(reset)
        },
        Some("pwd") => Command::Unknown("pwd".to_string()),
        Some("date") => Command::Unknown("date".to_string()),
        Some("uname") => Command::Unknown("uname".to_string()),
        Some(other) => {
            // Check if it's an installed binary/application
            let known_bins = ["foot", "lain", "lain_music", "about", "guild", "navi"];
            if known_bins.contains(&other) {
                Command::Unknown(format!("RUN:{}", other))
            } else {
                Command::Unknown(other.to_string())
            }
        },
        None => Command::Unknown(String::new()),
    }
}

fn run_command(
    cmd: Command,
    lines: UseStateHandle<Vec<HashMap<String, Vec<String>>>>
) {

    match cmd {
        Command::Ls => {
            let mut new_blocks = (*lines).clone();

            let mut block = HashMap::new();
            block.insert("ls".into(), vec!["@Proxy README.md".into()]);

            new_blocks.push(block);
            lines.set(new_blocks);
        }

        Command::Cat(txt) => {
            let mut new_blocks = (*lines).clone();

            let mut block = HashMap::new();
            if txt == "README.md" {
                block.insert("cat".into(), vec![txt.to_string()]);
            } else {
                block.insert("cat".into(), vec!["Arquivo não encontrado".into()]);
            }

            new_blocks.push(block);
            lines.set(new_blocks);
        }

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
            if cmd.is_empty() {
                block.insert("".into(), vec![]);
            } else if cmd.starts_with("RUN:") {
                let bin_name = cmd.trim_start_matches("RUN:").to_string();
                block.insert("exec".into(), vec![format!("Iniciando {}...", bin_name)]);
            } else if cmd == "pwd" {
                block.insert(cmd, vec!["/home".into()]);
            } else if cmd == "date" {
                block.insert(cmd, vec![chrono::Local::now().format("%a %b %d %H:%M:%S %Y").to_string()]);
            } else if cmd == "uname" {
                block.insert(cmd, vec!["navi 0.1.0 x86_64 GNU/Linux".into()]);
            } else {
                block.insert(cmd.clone(), vec![format!("Comando não encontrado: {}", cmd)]);
            }

            new_blocks.push(block);
            lines.set(new_blocks);
        }

        Command::Echo(txt) => {
            let mut new_blocks = (*lines).clone();

            let mut block = HashMap::new();
            block.insert("echo".into(), vec![txt]);

            new_blocks.push(block);
            lines.set(new_blocks);
        }

        Command::Help => {
            let mut new_blocks = (*lines).clone();

            let mut block = HashMap::new();
            block.insert("help".into(), vec![
                "Comandos disponíveis:".to_string(),
                "  ls        - lista arquivos".to_string(),
                "  cat <arq> - exibe arquivo".to_string(),
                "  clear     - limpa tela".to_string(),
                "  whoami    - exibe usuário".to_string(),
                "  pwd       - diretório atual".to_string(),
                "  date      - data e hora".to_string(),
                "  uname     - info do sistema".to_string(),
                "  echo <txt>- exibe texto".to_string(),
                "  neofetch  - info do sistema".to_string(),
                "  help      - exibe ajuda".to_string(),
                "".to_string(),
                "Apps: foot, lain, lain_music, about, guild".to_string(),
            ]);

            new_blocks.push(block);
            lines.set(new_blocks);
        },


    }
}

#[component]
pub fn Foot(props: &PropsBinare) -> Html {
    let output_history = use_state(Vec::<HashMap<String, Vec<String>>>::new);
    let input_value = use_state(String::new);
    let state = use_context::<UseStateHandle<Management>>().expect("No ctx found.");

    {
        let blocks = output_history.clone();
        use_effect_with((), move |_| {
            run_command(Command::Neofetch, blocks);
            || ()
        });
    }

    let window_props_terminal = yew::props! {
        PropsWindowLucy {
            name_window: "foot".to_string(),
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
        let state = state.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if input_value.is_empty() {
                return;
            }

            let command_str = input_value.as_str();
            let command_cli = parse_command(command_str);
            
            // Handle running binaries
            if let Command::Unknown(ref cmd) = command_cli
                && cmd.starts_with("RUN:")
            {
                let bin_name = cmd.trim_start_matches("RUN:").to_string();
                let random_pid = (Math::random() * 99999_f64) as usize;
                let current_pid = random_pid.to_string();
                set_item(current_pid.as_str(), bin_name.as_str());
                state.set(Management {
                    pid: get_all_storage_values(),
                    soft_current: Some(bin_name.clone()),
                });
                let mut new_blocks = (*output_history).clone();
                let mut block = HashMap::new();
                block.insert("exec".into(), vec![format!("Iniciando {}...", bin_name)]);
                new_blocks.push(block);
                output_history.set(new_blocks);
                input_value.set(String::new());
                return;
            }
            
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
