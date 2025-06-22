use std::collections::HashMap;
use ic_cdk::storage;
use ic_cdk_macros::{query, update};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static ITEMS: Lazy<Mutex<HashMap<u32, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[update]
fn create_item(id: u32, value: String) {
    let mut items = ITEMS.lock().unwrap();
    items.insert(id, value);
}

#[query]
fn read_item(id: u32) -> Option<String> {
    let items = ITEMS.lock().unwrap();
    items.get(&id).cloned()
}

#[update]
fn update_item(id: u32, new_value: String) {
    let mut items = ITEMS.lock().unwrap();
    if items.contains_key(&id) {
        items.insert(id, new_value);
    }
}

#[update]
fn delete_item(id: u32) {
    let mut items = ITEMS.lock().unwrap();
    items.remove(&id);
}
