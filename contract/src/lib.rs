use near_sdk::{
	near_bindgen, AccountId, BorshStorageKey, PanicOnDefault,
	borsh::{BorshDeserialize, BorshSerialize},
	collections::{UnorderedMap},
    serde::{Deserialize, Serialize}
};

mod utils;
mod object;

use crate::utils::*;
use crate::object::*;

#[derive(BorshSerialize, BorshStorageKey)]
#[borsh(crate = "near_sdk::borsh")]
enum StorageKey {
		Name
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Contract {
	owner_id: AccountId,
	object: UnorderedMap<u64, VObject>
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
			owner_id,
			object: UnorderedMap::new(StorageKey::Name),
        }
    }

    pub fn get_objects(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<(u64, ObjectOutput)> {
		 unordered_map_pagination(&self.object, from_index, limit)
    }
}
