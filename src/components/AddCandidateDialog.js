import React, { useState } from "react";
import TextField from '@mui/material/TextField';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogTitle from '@mui/material/DialogTitle';
import Button from '@mui/material/Button';

export default function AddCandidateDialog({ open, setOpen, addCandidate, fetchCandidates }) {
  const [name, setName] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [nameError, setNameError] = useState(false);
  const handleClose = () => {
    setName("");
    setNameError(false);
    setOpen(false);
  }

  const doAddCandidate = () => {
    setNameError(false);
    if (!name || name === "") {
      setNameError(true);
      return;
    }
    setIsLoading(true);
    addCandidate({name}).then(res => {
      setIsLoading(false);
      fetchCandidates();
      setOpen(false);
    });
  }

  return (
    <Dialog open={open} onClose={handleClose}>
      <DialogTitle>Add candidate</DialogTitle>
      <DialogContent>
        <TextField
          error={nameError}
          helperText={nameError ? "Candidate name cannot be empty" : ""}
          onChange={(e) => setName(e.target.value)}
          autoFocus
          margin="dense"
          id="candidateName"
          label="Candidate name"
          type="input"
          fullWidth
          variant="standard"
        />
      </DialogContent>
      <DialogActions>
        <Button disabled={isLoading} onClick={handleClose}>Cancel</Button>
        <Button disabled={isLoading} onClick={doAddCandidate}>{!isLoading ? "Add" : "Waiting"}</Button>
      </DialogActions>
    </Dialog>
  );
}
