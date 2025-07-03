use std::sync::Arc;

use crate::{background_heal_ops::HealRoutine, heal_ops::AllHealState, mrf::MRFState};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_BackgroundHealRoutine: Arc<HealRoutine> = HealRoutine::new();
    pub static ref GLOBAL_MRFState: Arc<MRFState> = Arc::new(MRFState::new());
    pub static ref GLOBAL_BackgroundHealState: Arc<AllHealState> = AllHealState::new(false);
}
