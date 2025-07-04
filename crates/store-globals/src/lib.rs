// Copyright 2024 RustFS Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rustfs_policy::auth::Credentials;
use std::sync::OnceLock;

static GLOBAL_ACTIVE_CRED: OnceLock<Credentials> = OnceLock::new();

pub fn init_global_action_cred(ak: Option<String>, sk: Option<String>) {
    let ak = {
        if let Some(k) = ak {
            k
        } else {
            rustfs_utils::string::gen_access_key(20).unwrap_or_default()
        }
    };

    let sk = {
        if let Some(k) = sk {
            k
        } else {
            rustfs_utils::string::gen_secret_key(32).unwrap_or_default()
        }
    };

    GLOBAL_ACTIVE_CRED
        .set(Credentials {
            access_key: ak,
            secret_key: sk,
            ..Default::default()
        })
        .unwrap();
}

pub fn get_global_action_cred() -> Option<Credentials> {
    GLOBAL_ACTIVE_CRED.get().cloned()
}
