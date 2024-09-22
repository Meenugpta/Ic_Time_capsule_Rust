use candid::{CandidType, Deserialize};
use ic_cdk::api::time;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct TimeCapsule {
    owner: String,
    message: String,
    unlock_time: u64,
}

type TimeCapsuleStore = HashMap<u64, TimeCapsule>;

thread_local! {
    static TIME_CAPSULES: RefCell<TimeCapsuleStore> = RefCell::new(HashMap::new());
}

#[update]
fn create_time_capsule(owner: String, message: String, unlock_time: u64) -> u64 {
    let id = TIME_CAPSULES.with(|capsules| {
        let mut capsules = capsules.borrow_mut();
        let id = capsules.len() as u64;
        capsules.insert(id, TimeCapsule {
            owner,
            message,
            unlock_time,
        });
        id
    });
    id
}

#[query]
fn get_time_capsule(id: u64) -> Option<TimeCapsule> {
    TIME_CAPSULES.with(|capsules| {
        let capsules = capsules.borrow();
        capsules.get(&id).cloned().and_then(|capsule| {
            if time() >= capsule.unlock_time {
                Some(capsule)
            } else {
                None
            }
        })
    })
}

#[query]
fn list_available_capsules() -> Vec<(u64, TimeCapsule)> {
    TIME_CAPSULES.with(|capsules| {
        let capsules = capsules.borrow();
        capsules
            .iter()
            .filter(|(_, capsule)| time() >= capsule.unlock_time)
            .map(|(id, capsule)| (*id, capsule.clone()))
            .collect()
    })
}

// Keep the greet function to maintain backwards compatibility
#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}