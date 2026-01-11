use std::sync::atomic::{AtomicU32, Ordering};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientId(pub u32);

static CLIENT_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

impl ClientId {
    pub fn new() -> Self {
        ClientId(CLIENT_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}
