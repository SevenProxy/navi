#[warn(unused_imports)]
#[warn(unused)]
mod storage;

pub use storage::{
    set_item,
    remove_item,
    get_all_storage_values,
};
