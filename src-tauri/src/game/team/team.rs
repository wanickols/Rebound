pub mod spawnpattern;

pub struct Team {
    pub team_id: u8,
    pub player_ids: Vec<EntityId>,
    color: (f32, f32, f32),
    spawn_pattern: SpawnPattern,
}

struct TeamManager {
    teams: Vec<Team>,
}
