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
use near_sdk::collections::LookupMap;
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Candidate {
  candidate_id: String,
  name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CandidateStats {
  candidate_id: String,
  name: String,
  total_vote: i32,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Voting {
  candidates: UnorderedMap<String, Candidate>,
  voter_track: LookupMap<String, i32>,
  voted_track: LookupMap<String, i32>,
}

impl Default for Voting {
  fn default() -> Self {
    Self {
      candidates: UnorderedMap::new(b"c".to_vec()),
      voter_track: LookupMap::new(b"vr".to_vec()),
      voted_track: LookupMap::new(b"vd".to_vec()),
    }
  }
}

#[near_bindgen]
impl Voting {
  pub fn add_candidate(&mut self, candidate: Candidate) -> bool {
    assert_eq!(candidate.candidate_id.is_empty(), false);
    assert_eq!(candidate.name.is_empty(), false);
    match self.candidates.get(&candidate.candidate_id) {
      Some(_) => {
        env::panic(format!("{} already exist", candidate.candidate_id).as_bytes());
      }
      None => {
        self.candidates.insert(&candidate.candidate_id, &candidate);
        self.voted_track.insert(&candidate.candidate_id, &0);
        return true;
      }
    }
  }

  pub fn view_single_candidate(&self, candidate_id: String) -> CandidateStats {
    match self.candidates.get(&candidate_id) {
      Some(candidate) => {
        match self.voted_track.get(&candidate_id) {
          Some(total_vote) => {
            return CandidateStats {
              candidate_id: candidate.candidate_id,
              name: candidate.name,
              total_vote: total_vote
            }
          }
          None => {
            env::panic(format!("Data asynchronous error").as_bytes());
          }
        }
      }
      None => {
        env::panic(format!("{} not found", candidate_id).as_bytes());
      }
    }
  }

  pub fn view_candidates(&self) -> Vec<CandidateStats> {
    let mut vec_ret = <Vec<CandidateStats>>::new();
    for (candidate_id, candidate) in self.candidates.iter() {
      let mut c_stats = CandidateStats {
        candidate_id: candidate.candidate_id,
        name: candidate.name,
        total_vote: 0,
      };
      match self.voted_track.get(&candidate_id) {
        Some(total_vote) => c_stats.total_vote = total_vote,
        None => {}
      }
      vec_ret.push(c_stats);
    }
    return vec_ret;
  }

  pub fn vote(&mut self, candidate_id: String) -> bool {
    let voter_id = env::signer_account_id();
    match self.voter_track.get(&voter_id) {
      Some(_) => {
        env::panic(format!("{} already voted in", voter_id).as_bytes());
      }
      None => match self.voted_track.get(&candidate_id) {
        Some(result) => {
          self.voted_track.insert(&candidate_id, &(result + 1));
          self.voter_track.insert(&voter_id, &1);
          return true;
        }
        None => {
          env::panic(format!("Candidate {} not found", candidate_id).as_bytes());
        }
      },
    }
  }

  pub fn check_voted(&mut self) -> bool {
    let voter_id = env::signer_account_id();
    match self.voter_track.get(&voter_id) {
      Some(_) => {
        return true;
      }
      None => {
        return false;
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

  #[test]
  fn should_add_candidate_success() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Voting::default();
    let ret = contract.add_candidate(Candidate {
      candidate_id: "0".to_string(),
      name: "Trump".to_string(),
    });
    assert_eq!(ret, true);
    let candidates = contract.view_single_candidate("0".to_string());
    assert_eq!(candidates.candidate_id, "0".to_string());
    assert_eq!(candidates.name, "Trump".to_string());
    assert_eq!(candidates.total_vote, 0);
  }

  #[test]
  #[should_panic]
  fn should_add_candidate_empty_name_fail() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Voting::default();
    contract.add_candidate(Candidate {
      candidate_id: "0".to_string(),
      name: "".to_string(),
    });
  }

  #[test]
  #[should_panic]
  fn should_add_candidate_empty_id_fail() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Voting::default();
    contract.add_candidate(Candidate {
      candidate_id: "".to_string(),
      name: "abc".to_string(),
    });
  }

  #[test]
  #[should_panic]
  fn should_add_candidate_exists_id_fail() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Voting::default();
    contract.add_candidate(Candidate {
      candidate_id: "0".to_string(),
      name: "Trump".to_string(),
    });
    contract.add_candidate(Candidate {
      candidate_id: "0".to_string(),
      name: "Biden".to_string(),
    });
  }

  #[test]
  fn test_view_candidates() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Voting::default();
    contract.add_candidate(Candidate {
      candidate_id: "0".to_string(),
      name: "Trump".to_string(),
    });
    contract.add_candidate(Candidate {
      candidate_id: "1".to_string(),
      name: "Biden".to_string(),
    });

    let ret = contract.view_candidates();
    assert_eq!(ret.len(), 2);
  }

  #[test]
  fn should_vote_candidate_success() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Voting::default();
    contract.add_candidate(Candidate {
      candidate_id: "0".to_string(),
      name: "Trump".to_string(),
    });
    let ret = contract.vote("0".to_string());
    assert_eq!(ret, true);
    let candidate = contract.view_single_candidate("0".to_string());
    assert_eq!(candidate.total_vote, 1);
  }

  #[test]
  #[should_panic]
  fn should_vote_candidate_already_fail() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Voting::default();
    contract.add_candidate(Candidate {
      candidate_id: "0".to_string(),
      name: "Trump".to_string(),
    });
    let ret = contract.vote("0".to_string());
    assert_eq!(ret, true);
    contract.vote("0".to_string());
  }

  #[test]
  #[should_panic]
  fn should_vote_candidate_not_exists_fail() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Voting::default();
    contract.add_candidate(Candidate {
      candidate_id: "0".to_string(),
      name: "Trump".to_string(),
    });
    contract.vote("1".to_string());
  }

  #[test]
  fn should_check_voted_return_false() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Voting::default();
    let ret = contract.check_voted();
    assert_eq!(ret, false);
  }

  #[test]
  fn should_check_voted_return_true() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Voting::default();
    contract.add_candidate(Candidate {
      candidate_id: "0".to_string(),
      name: "Trump".to_string(),
    });
    contract.vote("0".to_string());
    let ret = contract.check_voted();
    assert_eq!(ret, true);
  }
}
