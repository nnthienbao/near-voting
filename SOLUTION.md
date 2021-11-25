# **Voting smart contract build on NEAR**

## I) **Introduce**
This is my solution for [challenge #7](https://nearvember.near.org/challenge-7-voting-contract) for [nearvember contest](https://nearvember.near.org/)

## II) **My solution**
### 1) **Define data struct**
``` rust
// Each candidate will have an id (unique) and related information (currently only candidate name)
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Candidate {
  candidate_id: String,
  name: String,
}

// Structure to return voting information (including candidate information and that candidate's total votes)
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CandidateStats {
  candidate_id: String,
  name: String,
  total_vote: i32,
}

// Main data structure of smart contract
// candidates: map from candidate_id => Candidate
// voter_track: map used to check users can only vote once
// voted_track: map from candidate_id => total of votes for that candidate
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Voting {
  candidates: UnorderedMap<String, Candidate>,
  voter_track: LookupMap<String, i32>,
  voted_track: LookupMap<String, i32>,
}
```
#### **I designed the above Voting structure for the following reasons:**
- candidates: I use UnorderedMap because I need to iterate over the elements (for view_candidates function)
- voter_track: To check that the user can only vote once, I think using a LookupMap is the simplest
- voted_track: To track the total number of votes of a candidate, using a LookupMap will be simple. When a new candidate is voted, you just need to get it from the map and update the value

> Collection of NEAR sdk:
https://www.near-sdk.io/contract-structure/collections

### 2) **Implement Method**
**add_candidate**
``` rust
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
```
**view_candidates**
``` rust
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
```
**vote**
``` rust
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
```
**view_single_candidate**
``` rust
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
```

### 3) **Deploy to testnet**
Smart contract ID: **voting.rubikone.testnet**

### 4) **Testing**
- Set environment variable
    - export CONTRACT_NAME=voting.rubikone.testnet
    - export ACCOUNT_ID=&lt;your account id&gt;

- add_candidate (candidate_id is unique)
  ``` shell
  near call $CONTRACT_NAME add_candidate '{"candidate": {"candidate_id": "1", "name": "John Cena"}}' --accountId $ACCOUNT_ID
  ``` 
- view_candidates
  ``` shell
  near view $CONTRACT_NAME view_candidates '' --accountId $ACCOUNT_ID
  ```
- vote
  ``` shell
  near call $CONTRACT_NAME vote '{"candidate_id": "0"}' --accountId $ACCOUNT_ID
  ```
- view_single_candidate
  ``` shell
  near view $CONTRACT_NAME view_single_candidate '{"candidate_id": "0"}' --accountId $ACCOUNT_ID
  ```