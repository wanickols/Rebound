static ENTITY_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

impl EntityId {
    pub fn new() -> Self {
        EntityId(ENTITY_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}
