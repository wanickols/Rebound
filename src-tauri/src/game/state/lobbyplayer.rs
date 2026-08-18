use crate::game::state::entityid::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LobbyPlayer {
    pub player_id: EntityId,
    pub team_id: Option<u8>,
    //pub spawn_id: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LobbyState {
    pub expected_players: usize,
    pub players: Vec<LobbyPlayer>,
}

//I'm good with these in same file for now. if they get too big or have functions added might split );
