// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::env;
use tracing::warn;

pub fn read_env_var(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|e| {
        warn!(
            "Env variable {} is not set: {} - Fallback to {}",
            name, e, default
        );
        default.into()
    })
}
