"use strict";

const path = require('path');
const fs = require('fs');
const express = require('express');

const PORT = 8080;

const app = express();
const config = JSON.stringify(require('./config.js'));
app.get('*/config.js', function (req, res) {
  res.status(200).append('Content-Type', 'application/javascript').send('window.bityConfiguration = { exchangeClient: JSON.parse(`' + config + '`) };');
});
app.use(express.static('./'));
app.listen(PORT, function () {
  return console.log(`Bity client ready http://localhost:${PORT}`);
});
