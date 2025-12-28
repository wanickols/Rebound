use std::sync::atomic::{AtomicU32, Ordering};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

static ENTITY_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

impl EntityId {
    pub fn new() -> Self {
        EntityId(ENTITY_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}
