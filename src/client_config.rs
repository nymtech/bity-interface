use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DefaultOrder {
    input_currency: String,
    output_currency: String,
    input_amount: String,
    lock_output_address_when_prefilled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OAuthConfig {
    authorization_url: String,
    token_url: String,
    client_id: String,
    scopes: Vec<String>,
    redirect_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfig {
    pub client_id: String,
    pub exchange_api_url: String,
    pub legacy_v2_api_url: Option<String>,
    pub bity_dashboard_url: Option<String>,
    pub restrict_currencies_to_send: Vec<String>,
    pub restrict_currencies_to_receive: Vec<String>,
    pub default_order_parameters: DefaultOrder,
    pub oauth_config: OAuthConfig,
}
