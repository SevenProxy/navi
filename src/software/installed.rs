use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum TypeSoftware {
    Application,
    Tools,
    Internet,
}

#[derive(Clone)]
pub struct Software {
    pub name: String,
    pub icon: String,
    pub type_soft: TypeSoftware,
    pub shortcut: String,
}


#[derive(Clone)]
pub struct StartMenuSelect {
    pub name: String,
    pub icon: Html,
    pub select: Vec<Software>,
}

pub fn get_software() -> Vec<Software> {
    let applications_list = vec![
        Software {
            name: "foot".to_string(),
            icon: "https://cdn.terminaltrove.com/m/8edf284d-5ce4-4d52-ab31-350866aaa79e.png".to_string(),
            type_soft: TypeSoftware::Application,
            shortcut: "$b/foot".to_string(),
        },
        Software {
            name: "lain".to_string(),
            icon: "https://cdn.pfps.gg/pfps/3898-serial-experiments-lain-icon.png".to_string(),
            type_soft: TypeSoftware::Application,
            shortcut: "$b/lain".to_string(),
        },
        Software {
            name: "about".to_string(),
            icon: "".to_string(),
            type_soft: TypeSoftware::Tools,
            shortcut: "$b/about".to_string(),
        },
    ];

    applications_list
}

pub fn get_menu_select() -> Vec<StartMenuSelect> {
    let menu_list = vec![
        StartMenuSelect {
            name: "Tools".to_string(),
            icon: html!{},
            select: get_type_sotf(TypeSoftware::Tools),
        },
        StartMenuSelect {
            name: "Applications".to_string(),
            icon: html!{
                <svg fill="currentColor" class="bi bi-terminal-fill w-4 h-4 text-black" viewBox="0 0 16 16">
                    <path d="M0 3a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2zm9.5 5.5h-3a.5.5 0 0 0 0 1h3a.5.5 0 0 0 0-1m-6.354-.354a.5.5 0 1 0 .708.708l2-2a.5.5 0 0 0 0-.708l-2-2a.5.5 0 1 0-.708.708L4.793 6.5z"/>
                </svg>
            },
            select: get_type_sotf(TypeSoftware::Application),
        },
    ];

    menu_list
}

fn get_type_sotf(t: TypeSoftware) -> Vec<Software> {
    let mut result_soft = Vec::<Software>::new();

    for soft in get_software() {
        if soft.type_soft == t {
            result_soft.push(soft);
        }
    }

    result_soft
}


