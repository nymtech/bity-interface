"use strict";

/* An example configuration file. */
module.exports = {
  // OAuth client id
  clientId: "",
  // The URL to the Exchange API, something like https://exchange.api.bity.[io|com]
  exchangeApiUrl: "https://exchange.api.bity.com",
  // The URL to the legacy V2 API, something like https://bity.[io|com]
  legacyV2ApiUrl: "",
  // The URL which users will go to when the dashboard button is clicked
  bityDashboardUrl: "",
  // The URL which the bookmark service is located at
  bookmarksApiUrl: "",
  restrictCurrenciesToSend: ["BTC"],
  restrictCurrenciesToReceive: ["NYM"],
  defaultOrderParameters: {
    inputCurrency: "BTC",
    outputCurrency: "NYM",
    inputAmount: "0.0002",
  },
  // An OAuth configuration example.
  oauthConfig: {
    authorizationUrl: "https://connect.bity.com/oauth2/auth",
    tokenUrl: "https://connect.bity.com/oauth2/token",
    clientId: "",
    scopes: ["https://auth.bity.com/scopes/exchange.place"],
    redirectUrl: "https://buy.nymtech.net/",
  },
};
