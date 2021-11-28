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

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CandidateChart {
  candidate_id: String,
  name: String,
  data: Vec<PairTime>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PairTime {
  x: i64,
  y: i32,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Voting {
  candidates: UnorderedMap<String, Candidate>,
  voter_track: LookupMap<String, String>,
  voted_track: LookupMap<String, i32>,
  chart_tracking: LookupMap<String, UnorderedMap<i64, i32>>,
}

impl Default for Voting {
  fn default() -> Self {
    Self {
      candidates: UnorderedMap::new(b"c".to_vec()),
      voter_track: LookupMap::new(b"vr".to_vec()),
      voted_track: LookupMap::new(b"vd".to_vec()),
      chart_tracking: LookupMap::new(b"ct".to_vec()),
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
        self.chart_tracking.insert(
          &candidate.candidate_id,
          &UnorderedMap::new(env::sha256(&candidate.candidate_id.as_bytes())),
        );
        return true;
      }
    }
  }

  pub fn view_single_candidate(&self, candidate_id: String) -> CandidateStats {
    match self.candidates.get(&candidate_id) {
      Some(candidate) => match self.voted_track.get(&candidate_id) {
        Some(total_vote) => {
          return CandidateStats {
            candidate_id: candidate.candidate_id,
            name: candidate.name,
            total_vote: total_vote,
          }
        }
        None => {
          env::panic(format!("Data asynchronous error").as_bytes());
        }
      },
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
        Some(result) => match self.chart_tracking.get(&candidate_id) {
          Some(mut map_chart) => {
            let ts_in_millis =
              ((env::block_timestamp() / (86400 * 1000000000)) as i64) * (86400 * 1000);
            self.voted_track.insert(&candidate_id, &(result + 1));
            self.voter_track.insert(&voter_id, &candidate_id);
            match map_chart.get(&ts_in_millis) {
              Some(total) => {
                map_chart.insert(&ts_in_millis, &(total + 1));
              }
              None => {
                map_chart.insert(&ts_in_millis, &1);
              }
            }
            self.chart_tracking.insert(&candidate_id, &map_chart);
            return true;
          }
          None => {
            env::panic(format!("Map chart for candidate {} not found", candidate_id).as_bytes());
          }
        },
        None => {
          env::panic(format!("Candidate {} not found", candidate_id).as_bytes());
        }
      },
    }
  }

  pub fn check_voted(&self, account_id: String) -> Option<Candidate> {
    match self.voter_track.get(&account_id) {
      Some(candidate_id) =>  {
        return self.candidates.get(&candidate_id);
      },
      None => {
        return None
      }
    }
  }

  pub fn get_chart(&self) -> Vec<CandidateChart> {
    let mut vec_ret = <Vec<CandidateChart>>::new();
    for (candidate_id, candidate) in self.candidates.iter() {
      let mut c_chart = CandidateChart {
        candidate_id: candidate.candidate_id,
        name: candidate.name,
        data: <Vec<PairTime>>::new(),
      };
      match self.chart_tracking.get(&candidate_id) {
        Some(map_tracking) => {
          for (timestamp, value) in map_tracking.iter() {
            c_chart.data.push(PairTime {
              x: timestamp,
              y: value,
            });
          }
          vec_ret.push(c_chart);
        }
        None => {
          env::panic(format!("Candidate {} not found", candidate_id).as_bytes());
        }
      }
    }
    return vec_ret;
  }

  pub fn vote_fake(&mut self, candidate_id: String, timestamp: i64) -> bool {
    let signer_id = env::signer_account_id();
    assert_eq!(signer_id, "rubikone.testnet".to_string());

    match self.voted_track.get(&candidate_id) {
      Some(result) => match self.chart_tracking.get(&candidate_id) {
        Some(mut map_chart) => {
          self.voted_track.insert(&candidate_id, &(result + 1));
          match map_chart.get(&timestamp) {
            Some(total) => {
              map_chart.insert(&timestamp, &(total + 1));
            }
            None => {
              map_chart.insert(&timestamp, &1);
            }
          }
          self.chart_tracking.insert(&candidate_id, &map_chart);
          return true;
        }
        None => {
          env::panic(format!("Map chart for candidate {} not found", candidate_id).as_bytes());
        }
      },
      None => {
        env::panic(format!("Candidate {} not found", candidate_id).as_bytes());
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
      block_timestamp: 1638040621000000000,
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

  fn get_context_rubikone(input: Vec<u8>, is_view: bool) -> VMContext {
    VMContext {
      current_account_id: "alice_near".to_string(),
      signer_account_id: "rubikone.testnet".to_string(),
      signer_account_pk: vec![0, 1, 2],
      predecessor_account_id: "carol_near".to_string(),
      input,
      block_index: 0,
      block_timestamp: 1638040621000000,
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
    let contract = Voting::default();
    let ret = contract.check_voted("bob_near".to_string());
    assert_eq!(ret.is_none(), true);
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
    let ret = contract.check_voted("bob_near".to_string());
    assert_eq!(ret.is_some(), true);
    let candidate = ret.unwrap();
    assert_eq!(candidate.candidate_id, "0".to_string());
    assert_eq!(candidate.name, "Trump".to_string());
  }

  #[test]
  fn should_get_chart_return_vec_empty() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let contract = Voting::default();
    let ret = contract.get_chart();
    assert_eq!(ret.len(), 0);
  }

  #[test]
  fn should_get_chart_return_vec_with_size_true() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Voting::default();
    contract.add_candidate(Candidate {
      candidate_id: "0".to_string(),
      name: "Trump".to_string(),
    });
    let mut ret = contract.get_chart();
    assert_eq!(ret.len(), 1);

    contract.add_candidate(Candidate {
      candidate_id: "1".to_string(),
      name: "John cena".to_string(),
    });
    ret = contract.get_chart();
    assert_eq!(ret.len(), 2);
  }

  #[test]
  fn should_get_chart_return_with_vote_true() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Voting::default();
    contract.add_candidate(Candidate {
      candidate_id: "0".to_string(),
      name: "Trump".to_string(),
    });

    let mut ret = contract.get_chart();
    assert_eq!(ret.len(), 1);
    assert_eq!(ret[0].data.len(), 0);
    assert_eq!(contract.vote("0".to_string()), true);
    ret = contract.get_chart();
    assert_eq!(ret.len(), 1);
    assert_eq!(ret[0].data.len(), 1);
    assert_eq!(ret[0].data[0].x, 1637971200000);
    assert_eq!(ret[0].data[0].y, 1);
  }

  #[test]
  #[should_panic]
  pub fn should_vote_fake_return_panic() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Voting::default();
    contract.add_candidate(Candidate {
      candidate_id: "0".to_string(),
      name: "Trump".to_string(),
    });
    contract.vote_fake("0".to_string(), 1637971200000);
  }

  #[test]
  pub fn should_vote_fake_return_success() {
    let context = get_context_rubikone(vec![], false);
    testing_env!(context);
    let mut contract = Voting::default();
    contract.add_candidate(Candidate {
      candidate_id: "0".to_string(),
      name: "Trump".to_string(),
    });
    contract.vote_fake("0".to_string(), 1637971200000);
    contract.vote_fake("0".to_string(), 1637971200000);
    let chart = contract.get_chart();
    assert_eq!(chart.len(), 1);
    assert_eq!(chart[0].data.len(), 1);
    assert_eq!(chart[0].data[0].x, 1637971200000);
    assert_eq!(chart[0].data[0].y, 2);

    contract.vote_fake("0".to_string(), 1638057600000);
    let chart2 = contract.get_chart();
    assert_eq!(chart2.len(), 1);
    assert_eq!(chart2[0].data.len(), 2);
    assert_eq!(chart2[0].data[1].x, 1638057600000);
    assert_eq!(chart2[0].data[1].y, 1);
  }
}
