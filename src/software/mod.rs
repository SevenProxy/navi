mod installed;
mod local;

pub use installed::{
    TypeSoftware,
    Software,
    get_software,

    StartMenuSelect,
    get_menu_select,
};
pub use local::LocalSoftware;
