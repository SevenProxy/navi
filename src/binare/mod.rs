pub mod foot;
pub mod lain;
pub mod lain_music;
pub mod about;
pub mod guild;
pub mod bin;
pub mod terminal;

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PropsBinare {
    pub pid: String,
}
