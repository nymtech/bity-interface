// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{net::IpAddr, sync::Arc};

use axum::{
    extract::State,
    http::{HeaderMap, Request},
    middleware::Next,
    response::{Redirect, Response},
};
use maxminddb::geoip2::Country;
use tracing::{debug, error, info, instrument, warn};

use crate::AppState;

const IP_HEADER: &str = "X-Real-IP";

#[instrument(skip_all, level = "debug")]
pub async fn ip_check<B>(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, Redirect> {
    let ip_header = headers
        .get(IP_HEADER)
        .and_then(|header| header.to_str().ok());
    let ip = match ip_header {
        Some(ip) => ip,
        None => {
            warn!("Request header {} not set, skip IP checking", IP_HEADER);
            return Ok(next.run(req).await);
        }
    };

    let ip_addr: IpAddr = match ip.parse() {
        Ok(address) => address,
        Err(e) => {
            warn!("IP address parsing failed {}, skip IP checking", e);
            return Ok(next.run(req).await);
        }
    };

    // IP location lookup
    let result = match state.geoip_db.lookup::<Country>(ip_addr) {
        Ok(country) => country,
        Err(e) => {
            error!("geoIP lookup failed {}, skip IP checking", e);
            return Ok(next.run(req).await);
        }
    };

    // IP location lookup
    let data = match result.country {
        Some(data) => data,
        None => {
            warn!("geoIP lookup failed, skip IP checking");
            return Ok(next.run(req).await);
        }
    };

    // Suppose it is an FR IP if iso_code is not set
    let iso_alpha2 = String::from(data.iso_code.unwrap_or("FR"));
    debug!("IP located {}", iso_alpha2);

    // Finally if IP is located in US, reject the request
    if iso_alpha2 == "US" {
        info!("US IP detected, request kicked");
        return Err(Redirect::to("/403.html"));
        // return Err((StatusCode::FORBIDDEN, "US IP detected".into()));
    }
    Ok(next.run(req).await)
}
