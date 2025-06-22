use std::collections::HashMap;
use std::cell::RefCell;
use ic_cdk::api::stable::StableCell;
use ic_cdk_macros::{init, query, update};
use candid::{candid_method, CandidType, Deserialize};

thread_local! {
    static STORAGE: RefCell<HashMap<u32, String>> = RefCell::new(HashMap::new());
}

// Create item
#[update]
#[candid_method(update)]
fn create_item(id: u32, content: String) {
    STORAGE.with(|s| {
        s.borrow_mut().insert(id, content);
    });
}

// Read item
#[query]
#[candid_method(query)]
fn read_item(id: u32) -> Option<String> {
    STORAGE.with(|s| s.borrow().get(&id).cloned())
}

// Update item
#[update]
#[candid_method(update)]
fn update_item(id: u32, new_content: String) {
    STORAGE.with(|s| {
        s.borrow_mut().insert(id, new_content);
    });
}

// Delete item
#[update]
#[candid_method(update)]
fn delete_item(id: u32) {
    STORAGE.with(|s| {
        s.borrow_mut().remove(&id);
    });
}
