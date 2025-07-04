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
