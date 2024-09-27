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

// Helper function to validate the payload for creating a time capsule
fn validate_time_capsule_payload(owner: &String, message: &String, unlock_time: u64) -> Result<(), String> {
    if owner.trim().is_empty() {
        return Err("Owner is required".to_string());
    }
    if message.trim().is_empty() {
        return Err("Message is required".to_string());
    }
    if unlock_time <= time() {
        return Err("Unlock time must be in the future".to_string());
    }
    Ok(())
}

#[update]
fn create_time_capsule(owner: String, message: String, unlock_time: u64) -> Result<u64, String> {
    // Validate the payload before creating the time capsule
    validate_time_capsule_payload(&owner, &message, unlock_time)?;

    // Create a time capsule and return its ID
    let id = TIME_CAPSULES.with(|capsules| {
        let mut capsules = capsules.borrow_mut();
        let id = capsules.len() as u64; // Generate an ID based on the current count
        capsules.insert(id, TimeCapsule {
            owner,
            message,
            unlock_time,
        });
        id
    });
    Ok(id) // Return the ID of the newly created time capsule
}

#[query]
fn get_time_capsule(id: u64) -> Option<TimeCapsule> {
    TIME_CAPSULES.with(|capsules| {
        let capsules = capsules.borrow();
        capsules.get(&id).cloned().and_then(|capsule| {
            // Only return the capsule if the current time is past the unlock time
            if time() >= capsule.unlock_time {
                Some(capsule)
            } else {
                None // If unlock time is in the future, return None
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
            .filter(|(_, capsule)| time() >= capsule.unlock_time) // Only include capsules whose unlock time has passed
            .map(|(id, capsule)| (*id, capsule.clone())) // Return a list of ID and TimeCapsule pairs
            .collect()
    })
}

// A simple greet function, maintained for backwards compatibility
#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
