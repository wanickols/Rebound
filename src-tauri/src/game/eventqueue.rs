#[derive(Debug, Clone)]
pub enum GameEvent {
    GoalScored { team_id: u8 },
    ResetScore,
}

#[derive(Default)]
pub struct EventQueue {
    events: Vec<GameEvent>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, e: GameEvent) {
        self.events.push(e);
    }

    pub fn drain(&mut self) -> impl Iterator<Item = GameEvent> + '_ {
        self.events.drain(..)
    }
}
