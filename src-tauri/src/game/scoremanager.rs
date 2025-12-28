use tauri::window::Color;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Team {
    pub id: u8,
    pub name: String,
    pub color: Color,
    pub score: u8,
}

impl Team {
    pub fn reset(&mut self) {
        self.score = 0;
    }

    pub fn add_point(&mut self) {
        self.score += 1;
    }
}

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct ScoreManager {
    pub teams: Vec<Team>,
    can_score: bool,
    target_score: u8,
}

impl ScoreManager {
    pub fn new(team1: Team, team2: Team) -> Self {
        Self {
            teams: vec![team1, team2],
            can_score: true,
            target_score: 1,
        }
    }

    pub fn add_point(&mut self, team_id: u8) -> bool {
        if let Some(team) = self.teams.iter_mut().find(|t| t.id == team_id) {
            team.add_point();

            //game over check
            if team.score >= self.target_score {
                return true;
            }

            self.can_score = false;
            println!("Gooooooooooooal");
        }

        return false;
    }

    pub fn enable_score(&mut self) {
        self.can_score = true;
    }

    pub fn set_target_score(&mut self, target: u8) {
        self.target_score = target;
    }

    pub fn reset(&mut self) {
        for team in &mut self.teams {
            team.reset();
        }
        self.can_score = true;
    }
}
