use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Promise, PromiseOrValue};


// use near_sdk::collections::UnorderedMap;

near_sdk::setup_alloc!();

const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000; // 1 NEAR

#[near_bindgen]

// 1. Main Struct
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Roulette {
    pub owner_id: AccountId,
    // pub history: UnorderedMap<AccountId, Vector<u32>>,
}

// 2. Default Implementation
impl Default for Roulette {
    fn default() -> Self {
        panic!("Initialize!")        
    }
}

// 3. Core Logic
#[near_bindgen]
impl Roulette {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Invalid owner account");
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id,
            // balance: UnorderedMap::new(b"balance".to_vec()),
        }
    }

    // Logic is simple, choice from 1 to 9, the prize is from 1 to 9 as well
    // But you pay constantly 5 to play
    #[payable]
    pub fn play(&mut self) -> PromiseOrValue<String> {
        // check if the bet amount is correct
        let bet = env::attached_deposit();
        assert!(bet == 5_000_000_000_000_000_000_000_000, "Bet 5 only");

        // Thought to achieve randomness with timestamp, not sure how secure this is.
        let account_id = env::signer_account_id();
        let result: u64 = (env::block_timestamp() % 9 as u64) + 1 as u64;

        // User wins the choice, if not enough money, pay what ewe have        
        let winnings: u128 = result as u128 * ONE_NEAR; 
        let current_balance = env::account_balance();

        if current_balance <= winnings + ONE_NEAR {
            PromiseOrValue::Promise(Promise::new(account_id).transfer(current_balance))
        } else {
            PromiseOrValue::Promise(Promise::new(account_id).transfer(winnings))
        }
    }

    // You can use near view for avoid paying tx fee
    pub fn get_balance(&self) -> u128 {
        return env::account_balance();
    }

    // To-do: implement the history of each player
    //pub fn get_history(&self) -> u32 {
      //  return nul
    //}
}
// 4. Tests
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

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
            epoch_height: 0,
        }
    }

    // Test 1
    #[test]
    fn create_read_pair() {
        let context = get_context(vec![], false);
        testing_env!(context);
        
        // From string with conversion
        let alice: AccountId = "alice_near".parse().unwrap();

        let mut contract = Roulette::new(alice);
        contract.play();
        println!("The balance is {}", contract.get_balance());
    }
    // Test 2

}