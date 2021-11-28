import React, { useState, useEffect } from "react";
import Grid from "@mui/material/Grid";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Stack from "@mui/material/Stack";
import Big from 'big.js';
import _ from 'lodash'
import TableStatitics from "./TableStatitics";
import Chart from "./Chart";
import AddCandidateDialog from "./AddCandidateDialog";
import VotingDialog from "./VotingDialog";

const BOATLOAD_OF_GAS = Big(3).times(10 ** 13).toFixed();

export default function Form({ isSignIn, accountId, login, contract }) {
  const [candidates, setCandidates] = useState([]);
  const [votingFor, setVotingFor] = useState(null);
  const [chartData, setChartData] = useState([]);
  const [openAddCandidateDialog, setOpenAddCandidateDialog] = useState(false);

  const [openVotingDialog, setOpenVotingDialog] = useState(false);

  useEffect(() => {
    fetchCandidates();
    fetchChart();
    contract.check_voted({ account_id: accountId }).then((res) => {
      if (res) setVotingFor(res);
    });
  }, []);

  const fetchCandidates = () => {
    contract.view_candidates().then(candidatesRes => {
      const cSort = _.orderBy(candidatesRes, ['total_vote'], 'desc');
      setCandidates(cSort);
    });
  }

  const fetchChart = () => {
    contract.get_chart().then(data => {
      console.log(data);
      setChartData(data);
    })
  }

  const addCandidate = ({ name }) => {
    const candidate_id = Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
    return contract.add_candidate({candidate: {candidate_id, name}}, BOATLOAD_OF_GAS, "0");
  };

  const vote = ({candidate_id}) => {
    return contract.vote({candidate_id}, BOATLOAD_OF_GAS, "0");
  }

  return (
    <>
      <Grid
        container
        spacing={2}
        direction="column"
        alignItems="center"
        justifyContent="center"
      >
        <Grid item>
          <Typography variant="h5" component="div" sx={{ flexGrow: 1 }}>
            Who will be the next US president?
          </Typography>
        </Grid>
        <Grid item>
          <TableStatitics candidates={candidates} />
        </Grid>
        <Grid item>
          {isSignIn ? (
            <>
              {votingFor && (
                <Typography variant="h5" component="div" sx={{ flexGrow: 1 }}>
                  You voted for <strong>{votingFor.name}</strong>
                </Typography>
              )}
              <Stack direction="row" spacing={2}>
                <Button onClick={() => setOpenVotingDialog(true)} disabled={votingFor !== null} variant="outlined">
                  Press to vote
                </Button>
                <Button onClick={() => setOpenAddCandidateDialog(true)} variant="outlined">Add Candidate</Button>
              </Stack>
            </>
          ) : (
            <Button onClick={login} variant="outlined">
              Login to vote
            </Button>
          )}
        </Grid>
        <Grid item>
          <Chart data={chartData} />
        </Grid>
      </Grid>
      <AddCandidateDialog
        open={openAddCandidateDialog}
        setOpen={setOpenAddCandidateDialog}
        addCandidate={addCandidate}
        fetchCandidates={fetchCandidates}
        fetchChart={fetchChart}
      />
      <VotingDialog
        open={openVotingDialog}
        setOpen={setOpenVotingDialog}
        candidates={candidates}
        vote={vote}
        fetchCandidates={fetchCandidates}
        fetchChart={fetchChart}
      />
    </>
  );
}
