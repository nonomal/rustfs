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

use lazy_static::lazy_static;
use std::sync::OnceLock;

lazy_static! {
    pub static ref GLOBAL_RUSTFS_PORT: OnceLock<u16> = OnceLock::new();
}

/// Get the global rustfs port
pub fn get_global_rustfs_port() -> u16 {
    *GLOBAL_RUSTFS_PORT.get().unwrap_or(&9000)
}

/// Set the global rustfs port
pub fn set_global_rustfs_port(value: u16) {
    GLOBAL_RUSTFS_PORT.get_or_init(|| value);
}
