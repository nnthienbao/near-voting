import React, { useState } from "react";
import Radio from "@mui/material/Radio";
import RadioGroup from "@mui/material/RadioGroup";
import FormControlLabel from "@mui/material/FormControlLabel";
import FormControl from "@mui/material/FormControl";
import FormLabel from "@mui/material/FormLabel";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogTitle from "@mui/material/DialogTitle";
import Button from "@mui/material/Button";
import DialogContent from "@mui/material/DialogContent";

export default function VotingDialog({ open, setOpen, candidates, vote, fetchCandidates, fetchChart }) {
  const [isLoading, setIsLoading] = useState(false);
  const [candidateChoose, setCandidateChoose] = useState(null);

  const handleClose = () => {
    setOpen(false);
  };

  const doVote = () => {
    if (candidateChoose) {
      setIsLoading(true);
      vote({candidate_id: candidateChoose}).then(res => {
        fetchCandidates();
        fetchChart();
        setIsLoading(false);
        setOpen(false);
      })
    }
  };

  return (
    <Dialog open={open} onClose={handleClose}>
      <DialogTitle>Voting</DialogTitle>
      <DialogContent>
        <FormControl component="fieldset">
          <FormLabel component="legend">
            Who will be the next US president?
          </FormLabel>
          {candidates.length > 0 && (
            <RadioGroup
              aria-label="Voting"
              value={candidateChoose}
              onChange={(e) => setCandidateChoose(e.target.value)}
              name="radio-buttons-group"
            >
              {candidates.map((row) => (
                <FormControlLabel
                  key={row.candidate_id}
                  value={row.candidate_id}
                  control={<Radio />}
                  label={row.name}
                />
              ))}
            </RadioGroup>
          )}
        </FormControl>
      </DialogContent>
      <DialogActions>
        <Button disabled={isLoading} onClick={handleClose}>
          Cancel
        </Button>
        <Button disabled={isLoading} onClick={doVote}>
          {!isLoading ? "Vote" : "Waiting"}
        </Button>
      </DialogActions>
    </Dialog>
  );
}
