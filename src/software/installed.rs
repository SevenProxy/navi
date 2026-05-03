use std::collections::HashMap;

use yew::prelude::*;

#[derive(Eq, Hash, Clone, PartialEq)]
pub enum TypeSoftware {
    Application,
    Tools,
    //Internet,
}

#[derive(Clone)]
pub struct Software {
    pub name: String,
    pub icon: String,
    pub type_soft: TypeSoftware,
    //pub shortcut: String,
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
            icon: "public/img/foot.png".to_string(),
            type_soft: TypeSoftware::Application,
            //shortcut: "$b/foot".to_string(),
        },
        Software {
            name: "lain".to_string(),
            icon: "public/img/lain.jpg".to_string(),
            type_soft: TypeSoftware::Application,
            //shortcut: "$b/lain".to_string(),
        },
        Software {
            name: "about".to_string(),
            icon: "".to_string(),
            type_soft: TypeSoftware::Tools,
            //shortcut: "$b/about".to_string(),
        },
        Software {
            name: "guild".to_string(),
            icon: "".to_string(),
            type_soft: TypeSoftware::Tools,
            //shortcut: "$b/guild".to_string(),
        },
        Software {
            name: "lain_music".to_string(),
            icon: "public/img/lain_music.jpg".to_string(),
            type_soft: TypeSoftware::Application,
            //shortcut: "$b/lain_music".to_string(),
        },
    ];

    applications_list
}

pub fn get_menu_select() -> Vec<StartMenuSelect> {
    let list_soft = get_type_soft();

    let menu_list = vec![
        StartMenuSelect {
            name: "Tools".to_string(),
            icon: html!{
                <svg fill="currentColor" class="bi bi-gear-fill w-4 h-4 text-black" viewBox="0 0 16 16">
                    <path d="M9.405 1.05c-.413-1.4-2.397-1.4-2.81 0l-.1.34a1.464 1.464 0 0 1-2.105.872l-.31-.17c-1.283-.698-2.686.705-1.987 1.987l.169.311c.446.82.023 1.841-.872 2.105l-.34.1c-1.4.413-1.4 2.397 0 2.81l.34.1a1.464 1.464 0 0 1 .872 2.105l-.17.31c-.698 1.283.705 2.686 1.987 1.987l.311-.169a1.464 1.464 0 0 1 2.105.872l.1.34c.413 1.4 2.397 1.4 2.81 0l.1-.34a1.464 1.464 0 0 1 2.105-.872l.31.17c1.283.698 2.686-.705 1.987-1.987l-.169-.311a1.464 1.464 0 0 1 .872-2.105l.34-.1c1.4-.413 1.4-2.397 0-2.81l-.34-.1a1.464 1.464 0 0 1-.872-2.105l.17-.31c.698-1.283-.705-2.686-1.987-1.987l-.311.169a1.464 1.464 0 0 1-2.105-.872zM8 10.93a2.929 2.929 0 1 1 0-5.86 2.929 2.929 0 0 1 0 5.858z"/>
                </svg>
            },
            select: match list_soft.get(&TypeSoftware::Tools) {
                Some(t) => t.clone(),
                None => Vec::new(),
            },
        },
        StartMenuSelect {
            name: "Applications".to_string(),
            icon: html!{
                <svg fill="currentColor" class="bi bi-terminal-fill w-4 h-4 text-black" viewBox="0 0 16 16">
                    <path d="M0 3a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2zm9.5 5.5h-3a.5.5 0 0 0 0 1h3a.5.5 0 0 0 0-1m-6.354-.354a.5.5 0 1 0 .708.708l2-2a.5.5 0 0 0 0-.708l-2-2a.5.5 0 1 0-.708.708L4.793 6.5z"/>
                </svg>
            },
            select: match list_soft.get(&TypeSoftware::Application) {
                Some(t) => t.clone(),
                None => Vec::new(),
            },
        },
    ];

    menu_list
}

fn get_type_soft() -> HashMap<TypeSoftware, Vec<Software>> {
    let mut software_types = HashMap::<TypeSoftware, Vec<Software>>::new();

    for soft in get_software() {
        software_types
            .entry(soft.type_soft.clone())
            .or_insert_with(Vec::new)
            .push(soft)
    }

    software_types
}
