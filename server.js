"use strict";

const path = require("path");
const fs = require("fs");
const express = require("express");

require("dotenv").config();

const PORT = process.env.PORT;

if (!PORT) {
  console.error('Env variable PORT is not set');
  process.exit(1);
}

const app = express();
const config = JSON.stringify(require("./config.js"));
app.get("*/config.js", function (req, res) {
  res
    .status(200)
    .append("Content-Type", "application/javascript")
    .send(
      "window.bityConfiguration = { exchangeClient: JSON.parse(`" +
        config +
        "`) };"
    );
});
app.use(express.static("./"));
app.listen(PORT, function () {
  console.log(`Bity interface ready at http://localhost:${PORT}`);
});
