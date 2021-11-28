import React from "react";
import ReactDOM from "react-dom";
import App from "./App";
import { initContract } from "./utils";

window.nearInitPromise = initContract()
  .then(() => {
    ReactDOM.render(
      <App
        walletConnection={window.walletConnection}
        accountId={window.accountId}
        contract={window.contract}
      />,
      document.querySelector("#root")
    );
  })
  .catch(console.error);
