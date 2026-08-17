use crate::game::state::entityid::EntityId;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct LobbyPlayer {
    pub player_id: EntityId,
    pub team_id: Option<u8>,
    //pub spawn_id: Option<u8>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct LobbyState {
    pub expected_players: usize,
    pub players: Vec<LobbyPlayer>,
}

//I'm good with these in same file for now. if they get too big or have functions added might split ;)
