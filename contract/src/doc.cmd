pub struct Voting {
    // Map of poll id to voting options.
    polls: HashMap<String, VotingOptions>,
}

pub struct VotingOptions {
    // Author of the vote (account id).
    creator: String,
    // Unique voting id.
    poll_id: String,
    candidates: Map<candidate_id, Candidate>,
    votedTrack: Map<signer_id, Integer>
}

pub struct Candidate {
    candidate_id: String,
    name: String,
    total_vote: Number
}

view_polls() : [VotingOptions]
add_candidate(poll_id, {name}) : (true | false);
view_candidates(poll_id) : VoltingStats;
vote(poll_id, candidate_id) : (true | false);