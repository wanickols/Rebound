use crate::game::state::entityid::EntityId;

#[derive(Debug, Clone)]
pub enum GameEvent {
    GoalScored {
        team_id: u8,
    },
    ResetScore,
    TryGrab {
        player_id: EntityId,
    },
    Shoot {
        player_id: EntityId,
    },
    Place {
        player_id: EntityId,
        pos: (f32, f32),
    },
    Die {
        owner_id: EntityId,
        brick_index: usize,
    },
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
