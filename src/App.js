import "regenerator-runtime/runtime";
import React, { useState } from "react";
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import IconButton from '@mui/material/IconButton';
import MenuIcon from '@mui/icons-material/Menu';
import { login, logout } from "./utils";
import "./global.css";

import getConfig from "./config";
const { networkId } = getConfig(process.env.NODE_ENV || "development");

export default function App({ walletConnection, accountId, contract }) {
  const [isSignIn, setIsSignIn] = useState(false);
  React.useEffect(() => {
    // in this case, we only care to query the contract when signed in
    if (walletConnection.isSignedIn()) {
      setIsSignIn(true);
    }
  }, []);

  return (
    <AppBar position="static">
      <Toolbar>
        <IconButton
          size="large"
          edge="start"
          color="inherit"
          aria-label="menu"
          sx={{ mr: 2 }}
        >
          <MenuIcon />
        </IconButton>
        <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
          <h3>US presidential election</h3>
        </Typography>
        {isSignIn ? (
          <Button color="inherit" onClick={logout}>
            Logout
          </Button>
        ) : (
          <Button color="inherit" onClick={login}>
            Login
          </Button>
        )}
      </Toolbar>
    </AppBar>
  );
}
