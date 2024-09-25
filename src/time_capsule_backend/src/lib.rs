use candid::{CandidType, Deserialize};
use ic_cdk::api::{caller, time};
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

// Validation for empty strings and invalid unlock time
fn validate_input(owner: &String, message: &String, unlock_time: u64) -> Result<(), String> {
    if owner.trim().is_empty() || message.trim().is_empty() {
        return Err("Owner and message cannot be empty.".to_string());
    }
    if unlock_time <= time() {
        return Err("Unlock time must be in the future.".to_string());
    }
    Ok(())
}

// Create a new time capsule with validation
#[update]
fn create_time_capsule(owner: String, message: String, unlock_time: u64) -> Result<u64, String> {
    validate_input(&owner, &message, unlock_time)?;

    let id = TIME_CAPSULES.with(|capsules| {
        let mut capsules = capsules.borrow_mut();
        let id = capsules.len() as u64;
        capsules.insert(id, TimeCapsule {
            owner: owner.clone(),
            message,
            unlock_time,
        });
        id
    });

    Ok(id)
}

// Retrieve a time capsule if the unlock time has passed and the caller is the owner
#[query]
fn get_time_capsule(id: u64) -> Result<String, String> {
    TIME_CAPSULES.with(|capsules| {
        let capsules = capsules.borrow();
        if let Some(capsule) = capsules.get(&id) {
            if time() >= capsule.unlock_time {
                if capsule.owner == caller().to_string() {
                    Ok(format!("Message: {}", capsule.message))
                } else {
                    Err("Unauthorized: You are not the owner of this time capsule.".to_string())
                }
            } else {
                Err("Time capsule is still locked.".to_string())
            }
        } else {
            Err("Time capsule not found.".to_string())
        }
    })
}

// List all available capsules where unlock time has passed and return only if owned by the caller
#[query]
fn list_available_capsules() -> Vec<(u64, String)> {
    TIME_CAPSULES.with(|capsules| {
        let capsules = capsules.borrow();
        capsules
            .iter()
            .filter(|(_, capsule)| time() >= capsule.unlock_time && capsule.owner == caller().to_string())
            .map(|(id, capsule)| (*id, capsule.message.clone()))
            .collect()
    })
}

// Update a time capsule's message or unlock time (only by owner)
#[update]
fn update_time_capsule(id: u64, new_message: Option<String>, new_unlock_time: Option<u64>) -> Result<String, String> {
    TIME_CAPSULES.with(|capsules| {
        let mut capsules = capsules.borrow_mut();
        if let Some(capsule) = capsules.get_mut(&id) {
            if capsule.owner == caller().to_string() {
                if let Some(message) = new_message {
                    if message.trim().is_empty() {
                        return Err("Message cannot be empty.".to_string());
                    }
                    capsule.message = message;
                }

                if let Some(unlock_time) = new_unlock_time {
                    if unlock_time <= time() {
                        return Err("Unlock time must be in the future.".to_string());
                    }
                    capsule.unlock_time = unlock_time;
                }

                Ok("Time capsule updated successfully.".to_string())
            } else {
                Err("Unauthorized: You are not the owner of this time capsule.".to_string())
            }
        } else {
            Err("Time capsule not found.".to_string())
        }
    })
}

// Delete a time capsule (only by owner)
#[update]
fn delete_time_capsule(id: u64) -> Result<String, String> {
    TIME_CAPSULES.with(|capsules| {
        let mut capsules = capsules.borrow_mut();
        if let Some(capsule) = capsules.get(&id) {
            if capsule.owner == caller().to_string() {
                capsules.remove(&id);
                Ok("Time capsule deleted successfully.".to_string())
            } else {
                Err("Unauthorized: You are not the owner of this time capsule.".to_string())
            }
        } else {
            Err("Time capsule not found.".to_string())
        }
    })
}

// List all time capsules created by the caller (regardless of unlock status)
#[query]
fn list_capsules_by_owner() -> Vec<(u64, TimeCapsule)> {
    TIME_CAPSULES.with(|capsules| {
        let capsules = capsules.borrow();
        capsules
            .iter()
            .filter(|(_, capsule)| capsule.owner == caller().to_string())
            .map(|(id, capsule)| (*id, capsule.clone()))
            .collect()
    })
}

// Greet function for compatibility
#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

ic_cdk::export_candid!();