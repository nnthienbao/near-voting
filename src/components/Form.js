import React, { useState } from "react";
import Grid from "@mui/material/Grid";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Stack from "@mui/material/Stack";
import TableStatitics from "./TableStatitics";
import Chart from "./Chart"

export default function Form({ isSignIn, login }) {
  return (
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
        <TableStatitics />
      </Grid>
      <Grid item>
        {isSignIn
        ?<Stack direction="row" spacing={2}>
          <Button variant="outlined">Press to vote</Button>
          <Button variant="outlined">
            Add Candidate
          </Button>
        </Stack>
        : <Button onClick={login} variant="outlined">Login to vote</Button>
        }
      </Grid>
      <Grid item>
        <Chart />
      </Grid>
    </Grid>
  );
}
