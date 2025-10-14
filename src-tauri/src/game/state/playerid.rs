use serde::Serialize;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(pub u32, pub usize);

static PLAYER_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

impl PlayerId {
    // static atomic counter for generating unique IDs

    pub fn new(index: usize) -> Self {
        let id = PLAYER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        PlayerId(id, index)
    }
}
