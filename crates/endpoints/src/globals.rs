use lazy_static::lazy_static;
use std::sync::OnceLock;

use crate::EndpointServerPools;
use crate::PoolEndpoints;
use crate::SetupType;

lazy_static! {
    static ref GLOBAL_RUSTFS_PORT: OnceLock<u16> = OnceLock::new();
    static ref GLOBAL_IS_ERASURE: OnceLock<bool> = OnceLock::new();
    static ref GLOBAL_IS_DIST_ERASURE: OnceLock<bool> = OnceLock::new();
    static ref GLOBAL_IS_ERASURESD: OnceLock<bool> = OnceLock::new();
    static ref GLOBAL_ENDPOINTS: OnceLock<EndpointServerPools> = OnceLock::new();
}

/// Get the global rustfs port
pub fn get_global_rustfs_port() -> u16 {
    *GLOBAL_RUSTFS_PORT.get().unwrap_or(&9000)
}

/// Set the global rustfs port
pub fn set_global_rustfs_port(value: u16) {
    GLOBAL_RUSTFS_PORT.get_or_init(|| value);
}

pub fn is_dist_erasure() -> bool {
    let lock = GLOBAL_IS_DIST_ERASURE.get().unwrap_or(&false);
    *lock
}

pub fn is_erasure_sd() -> bool {
    let lock = GLOBAL_IS_ERASURESD.get().unwrap_or(&false);
    *lock
}

pub fn is_erasure() -> bool {
    let lock = GLOBAL_IS_ERASURE.get().unwrap_or(&false);
    *lock
}

pub fn update_erasure_type(setup_type: SetupType) {
    let is_dist_erasure = GLOBAL_IS_DIST_ERASURE.get_or_init(|| setup_type == SetupType::DistErasure);

    let is_erasure = if *is_dist_erasure {
        true
    } else {
        setup_type == SetupType::Erasure
    };

    GLOBAL_IS_ERASURE.get_or_init(|| is_erasure);

    GLOBAL_IS_ERASURESD.get_or_init(|| setup_type == SetupType::ErasureSD);
}

/// Get the global deployment id
pub fn set_global_endpoints(eps: Vec<PoolEndpoints>) {
    GLOBAL_ENDPOINTS
        .set(EndpointServerPools::from(eps))
        .expect("GLOBAL_Endpoints set failed")
}

/// Get the global endpoints
pub fn get_global_endpoints() -> EndpointServerPools {
    if let Some(eps) = GLOBAL_ENDPOINTS.get() {
        eps.clone()
    } else {
        EndpointServerPools::default()
    }
}
