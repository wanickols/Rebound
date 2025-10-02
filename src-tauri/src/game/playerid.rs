use serde::Serialize;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(pub u32);

static PLAYER_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

impl PlayerId {
    // static atomic counter for generating unique IDs

    pub fn new() -> Self {
        let id = PLAYER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        PlayerId(id)
    }
}
