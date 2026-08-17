use std::collections::HashMap;

use crate::game::{
    state::{
        entityid::EntityId,
        lobbyplayer::{LobbyPlayer, LobbyState},
        State,
    },
    util::Util,
};

pub struct World {
    pub entities: Vec<State>,
    entity_map: HashMap<EntityId, usize>,

    lobby_state: LobbyState,
}

impl World {
    pub fn new() -> Self {
        return World {
            entities: Vec::new(),
            entity_map: HashMap::new(),
            lobby_state: LobbyState {
                expected_players: 0,
                players: Vec::new(),
            },
        };
    }

    pub fn add_entity(&mut self, state: State) {
        self.entity_map.entry(state.entity_id).or_insert_with(|| {
            let idx = self.entities.len();
            self.entities.push(state);
            idx
        });
    }

    pub fn add_player(&mut self, state: State) {
        self.lobby_state.players.push(LobbyPlayer {
            player_id: state.entity_id,
            team_id: state.team_id,
        });
        self.add_entity(state);
    }

    pub fn remove_entity(&mut self, id: EntityId) {
        if let Some(&index) = self.entity_map.get(&id) {
            let last = self.entities.len() - 1;
            self.entities.swap_remove(index);

            self.entity_map.remove(&id);

            if index != last {
                let moved_id = self.entities[index].entity_id;
                self.entity_map.insert(moved_id, index);
            }
        }
    }

    pub fn remove_player(&mut self, entity_id: EntityId) {
        self.remove_entity(entity_id);
        if let Some(pos) = self
            .lobby_state
            .players
            .iter()
            .position(|player| player.player_id == entity_id)
        {
            self.lobby_state.players.swap_remove(pos); // replaces with last element, no shift
        }
    }

    pub fn remove_all(&mut self) {
        self.entities.clear();
        self.entity_map.clear();
        self.lobby_state.players.clear();
        println!("Removed all entities");
    }

    pub fn remove_all_non_players(&mut self) {
        for id in self.entity_map.keys().cloned().collect::<Vec<_>>() {
            if !self
                .lobby_state
                .players
                .iter()
                .any(|player| player.player_id == id)
            {
                self.remove_entity(id);
            }
        }
        println!(
            "Removed all non-player entities. Remaining entities: {}",
            self.entities.len()
        );
    }

    //Entity Accessors
    pub fn grab_entity(&mut self, id: EntityId) -> Option<&mut State> {
        let index = self.entity_map.get(&id).copied()?; // Option<usize>
        Some(&mut self.entities[index])
    }

    pub fn grab_two_entities(
        &mut self,
        id1: EntityId,
        id2: EntityId,
    ) -> Option<(&mut State, &mut State)> {
        let i = *self.entity_map.get(&id1)?;
        let j = *self.entity_map.get(&id2)?;

        Some(Util::two_mut(&mut self.entities, i, j))
    }

    pub fn get_lobby_state(&self) -> &LobbyState {
        &self.lobby_state
    }
    //Player Counts
    pub fn curr_player_count(&self) -> usize {
        return self.lobby_state.players.len();
    }

    pub fn expected_player_count(&self) -> usize {
        return self.lobby_state.expected_players;
    }

    pub fn set_expected_player_count(&mut self, count: usize) {
        self.lobby_state.expected_players = count;
    }

    pub fn reached_expected_player_count(&self) -> bool {
        return self.curr_player_count() >= self.expected_player_count();
    }
}
