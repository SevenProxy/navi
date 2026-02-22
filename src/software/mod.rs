mod store;
mod local;

pub use store::{
    TypeSoftware,
    Software,
    get_software,

    StartMenuSelect,
    get_menu_select,
};
pub use local::LocalSoftware;
