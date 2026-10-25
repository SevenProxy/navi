pub mod foot;
pub mod lain;
pub mod lain_music;
pub mod about;
pub mod guild;

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PropsBinare {
    pub pid: String,
}
