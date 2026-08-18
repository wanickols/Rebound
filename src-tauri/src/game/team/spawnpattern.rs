#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnPattern {
    Line,
    Diamond,
    Spread,
}

impl SpawnPattern {
    pub fn positions(&self, player_count: usize) -> Vec<(f32, f32)> {
        match self {
            SpawnPattern::Line => line_positions(player_count),
            SpawnPattern::Diamond => diamond_positions(player_count),
            SpawnPattern::Spread => spread_positions(player_count),
        }
    }
}

fn line_positions(player_count: usize) -> Vec<(f32, f32)> {
    match player_count {
        1 => vec![(0.0, 0.0)],
        2 => vec![(0.0, -25.0), (0.0, 25.0)],
        3 => vec![(0.0, -40.0), (0.0, 0.0), (0.0, 40.0)],
        4 => vec![(0.0, -45.0), (0.0, -15.0), (0.0, 15.0), (0.0, 45.0)],
        _ => Vec::new(),
    }
}

fn diamond_positions(player_count: usize) -> Vec<(f32, f32)> {
    match player_count {
        1 => vec![(0.0, 0.0)],
        2 => vec![(0.0, -30.0), (0.0, 30.0)],
        3 => vec![(-25.0, 0.0), (20.0, -30.0), (20.0, 30.0)],
        4 => vec![(-35.0, 0.0), (0.0, -30.0), (0.0, 30.0), (35.0, 0.0)],
        _ => Vec::new(),
    }
}

fn spread_positions(player_count: usize) -> Vec<(f32, f32)> {
    match player_count {
        1 => vec![(0.0, 0.0)],
        2 => vec![(-15.0, -30.0), (15.0, 30.0)],
        3 => vec![(-25.0, -35.0), (20.0, 0.0), (-25.0, 35.0)],
        4 => vec![(-25.0, -35.0), (25.0, -15.0), (25.0, 15.0), (-25.0, 35.0)],
        _ => Vec::new(),
    }
}
