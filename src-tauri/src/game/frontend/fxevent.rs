#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum FxEvent {
    BallWallHit { pos: (f32, f32), intensity: f32 },
    GoalScored { team_id: u8 },
    // PlaySound {
    //     kind: SoundKind,
    //     pos: (f32, f32),
    // },
}
