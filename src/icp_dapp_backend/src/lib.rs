use std::collections::HashMap;
use ic_cdk::storage;
use ic_cdk_macros::{query, update};
use once_cell::sync::Lazy;

static DATA: Lazy<std::sync::Mutex<HashMap<u32, String>>> = Lazy::new(|| {
    std::sync::Mutex::new(HashMap::new())
});

#[update]
fn create_item(id: u32, value: String) {
    let mut map = DATA.lock().unwrap();
    map.insert(id, value);
}

#[query]
fn read_item(id: u32) -> Option<String> {
    let map = DATA.lock().unwrap();
    map.get(&id).cloned()
}

#[update]
fn update_item(id: u32, value: String) {
    let mut map = DATA.lock().unwrap();
    if map.contains_key(&id) {
        map.insert(id, value);
    }
}

#[update]
fn delete_item(id: u32) {
    let mut map = DATA.lock().unwrap();
    map.remove(&id);
}


