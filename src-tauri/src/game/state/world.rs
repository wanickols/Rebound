use std::collections::HashMap;

use crate::game::{
    state::{entityid::EntityId, State},
    util::Util,
};

pub struct World {
    pub entities: Vec<State>,
    entity_map: HashMap<EntityId, usize>,

    pub player_list: Vec<EntityId>,
}

impl World {
    pub fn new() -> Self {
        return World {
            entities: Vec::new(),
            entity_map: HashMap::new(),
            player_list: Vec::new(),
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
        self.player_list.push(state.entity_id);
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
        if let Some(pos) = self.player_list.iter().position(|&id| id == entity_id) {
            self.player_list.swap_remove(pos); // replaces with last element, no shift
        }
    }

    pub fn remove_all(&mut self) {
        self.entities.clear();
        self.entity_map.clear();
        self.player_list.clear();
        println!("Removed all entities");
    }

    pub fn remove_all_non_players(&mut self) {
        for id in self.entity_map.keys().cloned().collect::<Vec<_>>() {
            if !self.player_list.contains(&id) {
                self.remove_entity(id);
            }
        }
        println!(
            "Removed all non-player entities. Remaining entities: {}",
            self.entities.len()
        );
    }

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

    pub fn curr_player_count(&self) -> usize {
        return self.player_list.len();
    }
}
