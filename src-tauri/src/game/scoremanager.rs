use tauri::window::Color;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Team {
    pub id: u8,
    pub name: String,
    pub color: Color,
    pub score: u32,
}

impl Team {
    pub fn reset(&mut self) {
        self.score = 0;
    }

    pub fn add_point(&mut self) {
        self.score += 1;
    }
}

#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScoreManager {
    pub teams: Vec<Team>,
    can_score: bool,
}

impl ScoreManager {
    pub fn new(team1: Team, team2: Team) -> Self {
        Self {
            teams: vec![team1, team2],
            can_score: true,
        }
    }

    pub fn add_point(&mut self, team_id: u8) {
        if let Some(team) = self.teams.iter_mut().find(|t| t.id == team_id) {
            team.add_point();
            self.can_score = false;
            println!("Gooooooooooooal");
        }
    }

    pub fn enable_score(&mut self) {
        self.can_score = true;
    }

    pub fn get_score(&self, team_id: u8) -> Option<u32> {
        self.teams.iter().find(|t| t.id == team_id).map(|t| t.score)
    }

    pub fn reset(&mut self) {
        for team in &mut self.teams {
            team.reset();
        }
        self.can_score = true;
    }
}
