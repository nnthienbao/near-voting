import React, { useState } from "react";
import Table from "@mui/material/Table";
import { styled } from '@mui/material/styles';
import TableBody from "@mui/material/TableBody";
import TableCell, { tableCellClasses } from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import Paper from "@mui/material/Paper";

const StyledTableCell = styled(TableCell)(({ theme }) => ({
  [`&.${tableCellClasses.head}`]: {
    backgroundColor: '#d97d73',
    color: theme.palette.common.white,
  },
  [`&.${tableCellClasses.body}`]: {
    fontSize: 14,
  },
}));

const StyledTableRow = styled(TableRow)(({ theme }) => ({
  '&:nth-of-type(odd)': {
    backgroundColor: theme.palette.action.hover,
  },
  // hide last border
  '&:last-child td, &:last-child th': {
    border: 0,
  },
}));

export default function TableStatitics({ candidates }) {
  return (
    <TableContainer component={Paper}>
      <Table sx={{ minWidth: 480 }} aria-label="customized table">
        <TableHead>
          <TableRow>
            <StyledTableCell><strong>Candidate</strong></StyledTableCell>
            <StyledTableCell align="right"><strong>Total votes</strong></StyledTableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {candidates.map((row) => (
            <StyledTableRow key={row.candidate_id}>
              <StyledTableCell component="th" scope="row">
                {row.name}
              </StyledTableCell>
              <StyledTableCell align="right">{row.total_vote}</StyledTableCell>
            </StyledTableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
}
