/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
use near_sdk::{env, near_bindgen, Balance, Promise};
use std::collections::HashMap;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Welcome {
    records: HashMap<String, Balance>,
}

#[near_bindgen]
impl Welcome {
    #[payable]
    pub fn deposit(&mut self, hash: String) {
        let deposit: Balance = env::attached_deposit();
        self.records.insert(hash, deposit);
    }

    pub fn get_deposit(&self, hash: String) -> Balance {
        match self.records.get(&hash) {
            Some(deposit) => {
                *deposit
            }
            None => {
                0
            }
        }
    }

    pub fn withdraw(&mut self, hash: String) -> bool {
        match self.records.get(&hash.clone()) {
            Some(deposit) => {
                assert!(deposit > &0, "Missing deposit");
                let account_id = env::predecessor_account_id();
                Promise::new(account_id).transfer(*deposit);
                self.records.insert(hash, 0);
                true
            }
            None => {
                env::log(format!("Wrong key").as_bytes());
                false
            }
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    fn ntoy(near_amount: Balance) -> Balance {
        near_amount * 10u128.pow(24)
    }

    #[test]
    fn test_deposit() {
        let mut context = get_context(vec![], true);
        context.is_view = false;
        context.attached_deposit = ntoy(100);
        testing_env!(context.clone());

        let mut contract = Welcome::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        contract.deposit("secret".to_string());

        assert_eq!(
            ntoy(100),
            contract.get_deposit("secret".to_string())
        );
    }

    #[test]
    fn test_withdraw() {
        let mut context = get_context(vec![], true);
        context.is_view = false;
        context.attached_deposit = ntoy(100);
        testing_env!(context.clone());

        let mut contract = Welcome::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        contract.deposit("secret".to_string());

        assert_eq!(
            ntoy(100),
            contract.get_deposit("secret".to_string())
        );

        contract.withdraw("secret".to_string());
        assert_eq!(
            ntoy(0),
            contract.get_deposit("secret".to_string())
        );
    }
}
