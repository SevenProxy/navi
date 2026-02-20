mod cpu;
mod desktop_environment;
mod window_manager;
mod waybar;

pub mod local_storage;

pub use desktop_environment::Desktop;
pub use cpu::Process;
pub use window_manager::{
    PropsWindowLucy,
    WindowLucyRoot,
};
pub use waybar::Waybar;
