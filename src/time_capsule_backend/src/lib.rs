use candid::{CandidType, Deserialize};
use ic_cdk::api::time;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use thiserror::Error;

#[derive(Clone, Debug, CandidType, Deserialize)]

struct TimeCapsule {
	owner: String,
	message: String,
	unlock_time: u64,
}

type TimeCapsuleStore = HashMap<u64, TimeCapsule>;
thread_local! {
	static TIME_CAPSULES: RefCell<TimeCapsuleStore> = RefCell::new(HashMap::new());
	static NEXT_ID: AtomicU64 = AtomicU64::new(0);
}

#[derive(Error, Debug)]
enum TimeCapsuleError {
	#[error("Invalid input data")]
	InvalidInput,
	#[error("Time capsule already exists")]
	AlreadyExists,
}

#[update]
fn create_time_capsule(owner: String, message: String, unlock_time: u64) -> Result<u64, TimeCapsuleError> {
	if owner.is_empty() || message.is_empty() || unlock_time <= time() {
		return Err(TimeCapsuleError::InvalidInput);
	}

	let id = NEXT_ID.with(|next_id| next_id.fetch_add(1, Ordering::SeqCst));

	TIME_CAPSULES.with(|capsules| {

		let mut capsules = capsules.borrow_mut();
		if capsules.contains_key(&id) {
			return Err(TimeCapsuleError::AlreadyExists);
		}

		capsules.insert(id, TimeCapsule {
			owner,
			message,
			unlock_time,
		});
		Ok(id)
	})
}