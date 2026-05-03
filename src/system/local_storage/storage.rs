#![allow(dead_code)]

use std::collections::HashMap;
use web_sys::{
    window,
    Storage,
};

fn get_local_storage() -> Storage {
    window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
}


pub fn set_item(key: &str, value: &str) {
    let storage = get_local_storage();

    let _ = storage.set_item(key, value);
}

pub fn get_item(key: &str) -> Option<String> {
    let storage = get_local_storage();

    if let Ok(t) = storage.get_item(key) {
        return t
    }

    None
}


pub fn remove_item(key: &str) {
    let storage = get_local_storage();

    let _ = storage.remove_item(key);
}

pub fn get_all_storage_values() -> HashMap<String, String> {
    let mut all_items = HashMap::new();

    let storage = get_local_storage();
    let length = match storage.length() {
        Ok(n) => n,
        Err(_) => 0,
    };

    for i in 0..length {
        if let Ok(Some(key)) = storage.key(i) {
            if let Ok(Some(value)) = storage.get_item(&key) {
                all_items.insert(key, value);
            }
        }
    }

    all_items
}
